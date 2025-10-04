use actix_web::web::Data;
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use log::{error, info, warn};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use tokio::time::sleep;

use crate::storage::{JsonStorage, MenuSchedule, ScheduleRecurrence, ScheduleStatus};

/// A wrapper for MenuSchedule that implements Ord for use in BinaryHeap
#[derive(Debug, Clone)]
struct ScheduledEvent {
    schedule: MenuSchedule,
    execution_time: DateTime<Utc>,
}

impl PartialEq for ScheduledEvent {
    fn eq(&self, other: &Self) -> bool {
        self.execution_time == other.execution_time
    }
}

impl Eq for ScheduledEvent {}

impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        // BinaryHeap is a max-heap, so we reverse the comparison to make it a min-heap
        other.execution_time.cmp(&self.execution_time)
    }
}

/// Check if a schedule conflicts with any existing schedules
/// A conflict occurs if the time ranges overlap
pub fn has_schedule_conflict(
    schedule: &MenuSchedule,
    existing_schedules: &[MenuSchedule],
) -> Option<MenuSchedule> {
    for existing in existing_schedules {
        // Skip the schedule itself if updating
        if existing.id == schedule.id {
            continue;
        }

        // Check for time overlap
        if schedule.start_time <= existing.end_time && schedule.end_time >= existing.start_time {
            return Some(existing.clone());
        }
    }
    None
}

/// Starts the scheduler service that runs in the background
/// checking for due menu schedules and executing them
pub async fn start_scheduler(storage: Data<JsonStorage>) {
    info!("Starting scheduler service");

    // Spawn the scheduler task as a background process
    tokio::spawn(async move {
        run_scheduler(storage).await;
    });
}

/// Main scheduler loop that efficiently waits for the next schedule to execute
async fn run_scheduler(storage: Data<JsonStorage>) {
    // Load and sort all pending and active schedules
    let mut events = load_scheduled_events(&storage).await;

    loop {
        // Get the next schedule to execute
        if let Some(event) = events.peek() {
            let now = Utc::now();

            if event.execution_time <= now {
                // Event is due to execute now
                let event = events.pop().unwrap();

                if matches!(event.schedule.status, ScheduleStatus::Active) {
                    // Check if Active schedule has ended
                    handle_ended_active_schedule(&storage, &event.schedule).await;
                } else {
                    // Execute the pending schedule
                    if let Err(e) = execute_schedule(&storage, event.schedule).await {
                        error!("Failed to execute schedule: {}", e);
                    }
                }

                // Reload events to account for any recurring schedules that may have been updated
                events = load_scheduled_events(&storage).await;
            } else {
                // Calculate sleep duration to the next event with millisecond precision
                let sleep_duration = (event.execution_time - now)
                    .to_std()
                    .unwrap_or_else(|_| std::time::Duration::from_secs(0));

                // Sleep until the next event is due
                sleep(sleep_duration).await;
            }
        } else {
            // No events scheduled, wait for a bit before checking again
            // This could happen if all schedules are ended or deleted
            sleep(std::time::Duration::from_secs(1)).await;

            // Check again for new events
            events = load_scheduled_events(&storage).await;
        }
    }
}

/// Load all pending and active schedules into a priority queue
async fn load_scheduled_events(storage: &Data<JsonStorage>) -> BinaryHeap<ScheduledEvent> {
    let mut events = BinaryHeap::new();
    let schedules = match storage.get_menu_schedules() {
        Ok(schedules) => schedules,
        Err(e) => {
            error!("Failed to get menu schedules: {}", e);
            return events;
        }
    };

    let now = Utc::now();

    for schedule in schedules {
        match schedule.status {
            ScheduleStatus::Pending => {
                if schedule.start_time >= now {
                    // Schedule is pending and will run in the future
                    events.push(ScheduledEvent {
                        schedule: schedule.clone(),
                        execution_time: schedule.start_time,
                    });
                } else {
                    // Schedule is pending but already past due - execute immediately
                    // We'll execute these when we return to the main loop
                    events.push(ScheduledEvent {
                        schedule: schedule.clone(),
                        execution_time: now, // Mark as executable immediately
                    });
                }
            }
            ScheduleStatus::Active => {
                // Active schedules need to be checked for when they end
                events.push(ScheduledEvent {
                    schedule: schedule.clone(),
                    execution_time: schedule.end_time,
                });
            }
            _ => {
                // Other statuses don't require scheduling
            }
        }
    }

    events
}

/// Execute a pending schedule by updating menu items based on the associated preset
async fn execute_schedule(
    storage: &Data<JsonStorage>,
    mut schedule: MenuSchedule,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Get all schedules to check for conflicts
    let all_schedules = storage.get_menu_schedules()?;

    // Check for conflicts before executing
    if let Some(conflicting_schedule) = has_schedule_conflict(&schedule, &all_schedules) {
        warn!(
            "Schedule {} ({}) conflicts with {} ({}), skipping execution",
            schedule.name, schedule.id, conflicting_schedule.name, conflicting_schedule.id
        );
        // Update schedule status to Conflicted
        let mut conflicted_schedule = schedule.clone();
        conflicted_schedule.status = ScheduleStatus::Conflicted;
        conflicted_schedule.error_message = Some(format!(
            "Conflicts with schedule '{}' ({})",
            conflicting_schedule.name, conflicting_schedule.id
        ));
        if let Err(update_err) = storage.update_menu_schedule(schedule.id, conflicted_schedule) {
            error!(
                "Failed to update schedule status to Conflicted: {}",
                update_err
            );
        }
        return Ok(());
    }

    info!(
        "Executing pending schedule: {} ({})",
        schedule.name, schedule.id
    );

    // Set status to Active during execution
    schedule.status = ScheduleStatus::Active;
    schedule.updated_at = Utc::now();
    storage.update_menu_schedule(schedule.id, schedule.clone())?;

    // Get the associated preset
    let presets = storage.get_menu_presets()?;
    let preset = presets
        .into_iter()
        .find(|p| p.id == schedule.preset_id)
        .ok_or_else(|| {
            format!(
                "Preset with id {} not found for schedule {}",
                schedule.preset_id, schedule.id
            )
        })?;

    // Get all menu items
    let menu_items = storage.get_menu_items()?;

    // Update menu items based on the preset
    // Set is_available = true for items in the preset
    // Set is_available = false for items not in the preset
    for mut item in menu_items {
        item.is_available = preset.menu_item_ids.contains(&item.id);
        storage.update_menu_item(item.id, item)?;
    }

    // Update schedule status based on recurrence and end time
    let now = Utc::now();

    // Check if schedule has ended
    if schedule.end_time <= now {
        // Schedule has ended, mark as ended
        schedule.status = ScheduleStatus::Ended;
        schedule.updated_at = now;
        schedule.error_message = None;
    } else {
        // Schedule is still active, update based on recurrence
        match schedule.recurrence {
            ScheduleRecurrence::Daily
            | ScheduleRecurrence::Weekly
            | ScheduleRecurrence::Monthly => {
                // For recurring schedules, calculate next occurrence and set status to Pending
                if let Some(next_start) = calculate_next_occurrence(&schedule, now) {
                    // Check if next occurrence is before or at end time
                    if next_start <= schedule.end_time {
                        schedule.start_time = next_start;
                        schedule.status = ScheduleStatus::Pending;
                        schedule.updated_at = now;
                        schedule.error_message = None; // Clear any previous error
                    } else {
                        // Next occurrence would be after end time, mark as ended
                        schedule.status = ScheduleStatus::Ended;
                        schedule.updated_at = now;
                        schedule.error_message =
                            Some("Next occurrence is after schedule end time".to_string());
                    }
                } else {
                    // If we can't calculate next occurrence, mark as ended
                    schedule.status = ScheduleStatus::Ended;
                    schedule.updated_at = now;
                    schedule.error_message = Some("Cannot calculate next occurrence".to_string());
                }
            }
            ScheduleRecurrence::Custom => {
                // For custom recurrence, mark as ended after execution
                schedule.status = ScheduleStatus::Ended;
                schedule.updated_at = now;
                schedule.error_message = None;
            }
        }
    }

    // Update the schedule in storage
    storage.update_menu_schedule(schedule.id, schedule.clone())?;

    info!(
        "Successfully executed pending schedule: {} ({})",
        schedule.name, schedule.id
    );
    Ok(())
}

/// Update an active schedule to ended status
async fn handle_ended_active_schedule(storage: &Data<JsonStorage>, schedule: &MenuSchedule) {
    info!(
        "Active schedule {} has ended, setting to Ended",
        schedule.id
    );
    let mut ended_schedule = schedule.clone();
    ended_schedule.status = ScheduleStatus::Ended;
    ended_schedule.updated_at = Utc::now();
    ended_schedule.error_message = None;
    if let Err(update_err) = storage.update_menu_schedule(schedule.id, ended_schedule) {
        error!("Failed to update active schedule to Ended: {}", update_err);
    }
}

/// Calculate the next occurrence of a recurring schedule
fn calculate_next_occurrence(
    schedule: &MenuSchedule,
    _now: chrono::DateTime<Utc>,
) -> Option<chrono::DateTime<Utc>> {
    match schedule.recurrence {
        ScheduleRecurrence::Daily => {
            // Add one day
            Some(schedule.start_time + ChronoDuration::days(1))
        }
        ScheduleRecurrence::Weekly => {
            // Add one week
            Some(schedule.start_time + ChronoDuration::weeks(1))
        }
        ScheduleRecurrence::Monthly => {
            // For monthly, we add one month
            schedule
                .start_time
                .date_naive()
                .checked_add_months(chrono::Months::new(1))
                .map(|next_month| next_month.and_time(schedule.start_time.time()).and_utc())
        }
        ScheduleRecurrence::Custom => None, // Custom recurrence not implemented yet
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration as ChronoDuration;
    use uuid::Uuid;

    #[test]
    fn test_has_schedule_conflict_no_conflict() {
        let now = Utc::now();
        let schedule1 = MenuSchedule {
            id: uuid::Uuid::new_v4(),
            preset_id: uuid::Uuid::new_v4(),
            name: "Schedule 1".to_string(),
            description: "Test schedule 1".to_string(),
            start_time: now,
            end_time: now + ChronoDuration::hours(1),
            recurrence: ScheduleRecurrence::Daily,
            status: ScheduleStatus::Pending,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        let schedule2 = MenuSchedule {
            id: uuid::Uuid::new_v4(),
            preset_id: uuid::Uuid::new_v4(),
            name: "Schedule 2".to_string(),
            description: "Test schedule 2".to_string(),
            start_time: now + ChronoDuration::hours(2),
            end_time: now + ChronoDuration::hours(3),
            recurrence: ScheduleRecurrence::Daily,
            status: ScheduleStatus::Pending,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        let existing_schedules = vec![schedule2];
        let result = has_schedule_conflict(&schedule1, &existing_schedules);

        assert!(result.is_none());
    }

    #[test]
    fn test_has_schedule_conflict_with_conflict() {
        let now = Utc::now();
        let schedule1 = MenuSchedule {
            id: uuid::Uuid::new_v4(),
            preset_id: uuid::Uuid::new_v4(),
            name: "Schedule 1".to_string(),
            description: "Test schedule 1".to_string(),
            start_time: now,
            end_time: now + ChronoDuration::hours(2),
            recurrence: ScheduleRecurrence::Daily,
            status: ScheduleStatus::Pending,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        let schedule2 = MenuSchedule {
            id: uuid::Uuid::new_v4(),
            preset_id: uuid::Uuid::new_v4(),
            name: "Schedule 2".to_string(),
            description: "Test schedule 2".to_string(),
            start_time: now + ChronoDuration::hours(1),
            end_time: now + ChronoDuration::hours(3),
            recurrence: ScheduleRecurrence::Daily,
            status: ScheduleStatus::Pending,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        let existing_schedules = vec![schedule2.clone()];
        let result = has_schedule_conflict(&schedule1, &existing_schedules);

        assert!(result.is_some());
        assert_eq!(result.unwrap().id, schedule2.id);
    }

    #[test]
    fn test_has_schedule_conflict_same_id_ignored() {
        let now = Utc::now();
        let schedule = MenuSchedule {
            id: uuid::Uuid::new_v4(),
            preset_id: uuid::Uuid::new_v4(),
            name: "Schedule".to_string(),
            description: "Test schedule".to_string(),
            start_time: now,
            end_time: now + ChronoDuration::hours(1),
            recurrence: ScheduleRecurrence::Daily,
            status: ScheduleStatus::Pending,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        let existing_schedules = vec![schedule.clone()];
        let result = has_schedule_conflict(&schedule, &existing_schedules);

        assert!(result.is_none());
    }

    #[test]
    fn test_calculate_next_occurrence_daily() {
        let now = Utc::now();
        let schedule = MenuSchedule {
            id: uuid::Uuid::new_v4(),
            preset_id: uuid::Uuid::new_v4(),
            name: "Daily Schedule".to_string(),
            description: "Test daily schedule".to_string(),
            start_time: now,
            end_time: now + ChronoDuration::hours(1),
            recurrence: ScheduleRecurrence::Daily,
            status: ScheduleStatus::Pending,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        let next_occurrence = calculate_next_occurrence(&schedule, now).unwrap();
        assert_eq!(next_occurrence, now + ChronoDuration::days(1));
    }

    #[test]
    fn test_calculate_next_occurrence_weekly() {
        let now = Utc::now();
        let schedule = MenuSchedule {
            id: uuid::Uuid::new_v4(),
            preset_id: uuid::Uuid::new_v4(),
            name: "Weekly Schedule".to_string(),
            description: "Test weekly schedule".to_string(),
            start_time: now,
            end_time: now + ChronoDuration::hours(1),
            recurrence: ScheduleRecurrence::Weekly,
            status: ScheduleStatus::Pending,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        let next_occurrence = calculate_next_occurrence(&schedule, now).unwrap();
        assert_eq!(next_occurrence, now + ChronoDuration::weeks(1));
    }

    #[test]
    fn test_calculate_next_occurrence_monthly() {
        let now = chrono::DateTime::parse_from_rfc3339("2023-01-15T10:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let schedule = MenuSchedule {
            id: uuid::Uuid::new_v4(),
            preset_id: uuid::Uuid::new_v4(),
            name: "Monthly Schedule".to_string(),
            description: "Test monthly schedule".to_string(),
            start_time: now,
            end_time: now + ChronoDuration::hours(1),
            recurrence: ScheduleRecurrence::Monthly,
            status: ScheduleStatus::Pending,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        let next_occurrence = calculate_next_occurrence(&schedule, now).unwrap();
        let expected = chrono::DateTime::parse_from_rfc3339("2023-02-15T10:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(next_occurrence, expected);
    }

    #[test]
    fn test_calculate_next_occurrence_custom_returns_none() {
        let now = Utc::now();
        let schedule = MenuSchedule {
            id: uuid::Uuid::new_v4(),
            preset_id: uuid::Uuid::new_v4(),
            name: "Custom Schedule".to_string(),
            description: "Test custom schedule".to_string(),
            start_time: now,
            end_time: now + ChronoDuration::hours(1),
            recurrence: ScheduleRecurrence::Custom,
            status: ScheduleStatus::Pending,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        let next_occurrence = calculate_next_occurrence(&schedule, now);
        assert!(next_occurrence.is_none());
    }

    #[test]
    fn test_scheduled_event_ordering() {
        let now = Utc::now();
        let event1 = ScheduledEvent {
            schedule: MenuSchedule {
                id: uuid::Uuid::new_v4(),
                preset_id: uuid::Uuid::new_v4(),
                name: "Schedule 1".to_string(),
                description: "Test schedule 1".to_string(),
                start_time: now + ChronoDuration::hours(2),
                end_time: now + ChronoDuration::hours(3),
                recurrence: ScheduleRecurrence::Daily,
                status: ScheduleStatus::Pending,
                error_message: None,
                created_at: now,
                updated_at: now,
            },
            execution_time: now + ChronoDuration::hours(2),
        };

        let event2 = ScheduledEvent {
            schedule: MenuSchedule {
                id: uuid::Uuid::new_v4(),
                preset_id: uuid::Uuid::new_v4(),
                name: "Schedule 2".to_string(),
                description: "Test schedule 2".to_string(),
                start_time: now + ChronoDuration::hours(1),
                end_time: now + ChronoDuration::hours(2),
                recurrence: ScheduleRecurrence::Daily,
                status: ScheduleStatus::Pending,
                error_message: None,
                created_at: now,
                updated_at: now,
            },
            execution_time: now + ChronoDuration::hours(1),
        };

        // Since ScheduledEvent implements Ord with reverse comparison to make BinaryHeap a min-heap,
        // the event with earlier execution time should come first when popped from the heap
        let mut heap = BinaryHeap::new();
        heap.push(event1);
        heap.push(event2);

        let first_popped = heap.pop().unwrap();
        assert_eq!(first_popped.execution_time, now + ChronoDuration::hours(1));
    }

    #[tokio::test]
    async fn test_scheduler_conflict_detection() {
        use crate::scheduler::has_schedule_conflict;

        let now = Utc::now();

        // Create first schedule
        let schedule1 = MenuSchedule {
            id: Uuid::new_v4(),
            preset_id: Uuid::new_v4(),
            name: "Schedule 1".to_string(),
            description: "First schedule".to_string(),
            start_time: now,
            end_time: now + ChronoDuration::hours(2),
            recurrence: ScheduleRecurrence::Daily,
            status: ScheduleStatus::Pending,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        // Create overlapping schedule
        let schedule2 = MenuSchedule {
            id: Uuid::new_v4(),
            preset_id: Uuid::new_v4(),
            name: "Schedule 2".to_string(),
            description: "Second schedule".to_string(),
            start_time: now + ChronoDuration::hours(1), // Overlaps with schedule1
            end_time: now + ChronoDuration::hours(3),
            recurrence: ScheduleRecurrence::Daily,
            status: ScheduleStatus::Pending,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        // Create non-overlapping schedule
        let schedule3 = MenuSchedule {
            id: Uuid::new_v4(),
            preset_id: Uuid::new_v4(),
            name: "Schedule 3".to_string(),
            description: "Third schedule".to_string(),
            start_time: now + ChronoDuration::hours(4),
            end_time: now + ChronoDuration::hours(5),
            recurrence: ScheduleRecurrence::Daily,
            status: ScheduleStatus::Pending,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        // Test conflict detection
        let schedules_with_conflict = vec![schedule2.clone()];
        let conflict_result = has_schedule_conflict(&schedule1, &schedules_with_conflict);
        assert!(conflict_result.is_some());
        assert_eq!(conflict_result.unwrap().id, schedule2.id);

        // Test no conflict
        let schedules_no_conflict = vec![schedule3];
        let no_conflict_result = has_schedule_conflict(&schedule1, &schedules_no_conflict);
        assert!(no_conflict_result.is_none());

        // Test same ID is ignored
        let same_schedule = vec![schedule1.clone()];
        let same_id_result = has_schedule_conflict(&schedule1, &same_schedule);
        assert!(same_id_result.is_none());
    }

    #[tokio::test]
    async fn test_recurring_schedule_calculation() {
        use crate::scheduler::calculate_next_occurrence;

        let now = Utc::now();

        // Test daily recurrence
        let daily_schedule = MenuSchedule {
            id: Uuid::new_v4(),
            preset_id: Uuid::new_v4(),
            name: "Daily Schedule".to_string(),
            description: "Daily test schedule".to_string(),
            start_time: now,
            end_time: now + ChronoDuration::hours(1),
            recurrence: ScheduleRecurrence::Daily,
            status: ScheduleStatus::Pending,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        let next_daily = calculate_next_occurrence(&daily_schedule, now).unwrap();
        assert_eq!(next_daily, now + ChronoDuration::days(1));

        // Test weekly recurrence
        let weekly_schedule = MenuSchedule {
            id: Uuid::new_v4(),
            preset_id: Uuid::new_v4(),
            name: "Weekly Schedule".to_string(),
            description: "Weekly test schedule".to_string(),
            start_time: now,
            end_time: now + ChronoDuration::hours(1),
            recurrence: ScheduleRecurrence::Weekly,
            status: ScheduleStatus::Pending,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        let next_weekly = calculate_next_occurrence(&weekly_schedule, now).unwrap();
        assert_eq!(next_weekly, now + ChronoDuration::weeks(1));

        // Test monthly recurrence (using a date that allows for safe addition)
        let month_start = chrono::DateTime::parse_from_rfc3339("2023-01-15T10:00:00Z")
            .unwrap()
            .with_timezone(&Utc);

        let monthly_schedule = MenuSchedule {
            id: Uuid::new_v4(),
            preset_id: Uuid::new_v4(),
            name: "Monthly Schedule".to_string(),
            description: "Monthly test schedule".to_string(),
            start_time: month_start,
            end_time: month_start + ChronoDuration::hours(1),
            recurrence: ScheduleRecurrence::Monthly,
            status: ScheduleStatus::Pending,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        let next_monthly = calculate_next_occurrence(&monthly_schedule, now).unwrap();
        let expected_monthly = chrono::DateTime::parse_from_rfc3339("2023-02-15T10:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(next_monthly, expected_monthly);

        // Test custom recurrence (should return None)
        let custom_schedule = MenuSchedule {
            id: Uuid::new_v4(),
            preset_id: Uuid::new_v4(),
            name: "Custom Schedule".to_string(),
            description: "Custom test schedule".to_string(),
            start_time: now,
            end_time: now + ChronoDuration::hours(1),
            recurrence: ScheduleRecurrence::Custom,
            status: ScheduleStatus::Pending,
            error_message: None,
            created_at: now,
            updated_at: now,
        };

        let next_custom = calculate_next_occurrence(&custom_schedule, now);
        assert!(next_custom.is_none());
    }
}

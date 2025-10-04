use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parsing error: {0}")]
    TomlParse(String),
    #[error("Config validation error: {0}")]
    Validation(String),
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::TomlParse(err.to_string())
    }
}

// Admin Configuration Structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminConfig {
    pub metadata: ConfigMetadata,
    pub admin_users: Vec<AdminUser>,
    pub roles: HashMap<String, Role>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigMetadata {
    pub schema_version: String,
    pub config_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminUser {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login: Option<DateTime<Utc>>,
    #[serde(default)]
    pub roles: Vec<String>,
    #[serde(default = "default_true")]
    pub is_active: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Role {
    pub description: String,
    pub permissions: Vec<String>,
}

// Validation Rules Structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidationRules {
    pub metadata: ConfigMetadata,
    pub menu_items: MenuItemValidation,
    pub menu_presets: MenuPresetValidation,
    pub menu_schedules: MenuScheduleValidation,
    pub notices: NoticeValidation,
    pub admin_users: AdminUserValidation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuItemValidation {
    pub name_min_length: usize,
    pub name_max_length: usize,
    pub allow_duplicate_names: bool,
    pub description_min_length: usize,
    pub description_max_length: usize,
    pub valid_categories: Vec<String>,
    pub allergens: AllergenValidation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AllergenValidation {
    pub valid_allergens: Vec<String>,
    pub allow_custom_allergens: bool,
    pub custom_allergen_max_length: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuPresetValidation {
    pub name_min_length: usize,
    pub name_max_length: usize,
    pub allow_duplicate_names: bool,
    pub description_min_length: usize,
    pub description_max_length: usize,
    pub min_items: usize,
    pub max_items: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuScheduleValidation {
    pub name_min_length: usize,
    pub name_max_length: usize,
    pub description_min_length: usize,
    pub description_max_length: usize,
    pub min_schedule_duration_hours: usize,
    pub max_schedule_duration_days: usize,
    pub allow_overlapping_schedules: bool,
    pub check_preset_availability: bool,
    pub valid_recurrence: Vec<String>,
    pub valid_status: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoticeValidation {
    pub title_min_length: usize,
    pub title_max_length: usize,
    pub content_min_length: usize,
    pub content_max_length: usize,
    pub max_active_notices: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminUserValidation {
    pub username_min_length: usize,
    pub username_max_length: usize,
    pub username_pattern: String,
    pub password_min_length: usize,
    pub password_require_uppercase: bool,
    pub password_require_lowercase: bool,
    pub password_require_numbers: bool,
    pub password_require_special_chars: bool,
    pub password_special_chars: String,
}

// App Settings Structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub metadata: ConfigMetadata,
    pub app: AppConfig,
    pub localization: LocalizationConfig,
    pub security: SecurityConfig,
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub storage: StorageConfig,
    pub menu: MenuConfig,
    pub notifications: NotificationsConfig,
    pub ui: UiConfig,
    pub performance: PerformanceConfig,
    pub development: DevelopmentConfig,
    pub features: FeaturesConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub environment: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalizationConfig {
    pub default_language: String,
    pub timezone: String,
    pub date_format: String,
    pub time_format: String,
    pub supported_languages: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecurityConfig {
    pub session_timeout_minutes: u32,
    pub session_cookie_name: String,
    pub session_cookie_secure: bool,
    pub session_cookie_httponly: bool,
    pub session_cookie_same_site: String,
    pub max_login_attempts: u32,
    pub login_lockout_duration_minutes: u32,
    pub password_reset_token_expiry_hours: u32,
    pub require_2fa: bool,
    #[serde(rename = "2fa_issuer")]
    pub tfa_issuer: String,
    pub enable_cors: bool,
    pub cors_allowed_origins: Vec<String>,
    pub cors_max_age_seconds: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub max_connections: usize,
    pub keep_alive_seconds: u64,
    pub client_timeout_seconds: u64,
    pub shutdown_timeout_seconds: u64,
    pub max_request_size_mb: usize,
    pub max_json_payload_mb: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub log_to_console: bool,
    pub log_to_file: bool,
    pub log_file_path: String,
    pub log_rotation: String,
    pub max_log_file_size_mb: usize,
    pub max_log_files: usize,
    pub log_format: String,
    pub log_timestamp_format: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageConfig {
    pub data_directory: String,
    pub backup_directory: String,
    pub enable_auto_backup: bool,
    pub backup_interval_hours: u32,
    pub max_backup_count: usize,
    pub menu_items_file: String,
    pub notices_file: String,
    pub menu_presets_file: String,
    pub menu_schedules_file: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuConfig {
    pub enable_scheduling: bool,
    pub schedule_check_interval_seconds: u64,
    pub auto_activate_schedules: bool,
    pub default_item_availability: bool,
    pub track_item_history: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationsConfig {
    pub enable_email_notifications: bool,
    pub enable_push_notifications: bool,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from_address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiConfig {
    pub theme: String,
    pub primary_color: String,
    pub enable_animations: bool,
    pub enforce_accessibility_checks: bool,
    pub wcag_level: String,
    pub minimum_contrast_ratio: f32,
    pub items_per_page: usize,
    pub show_allergen_icons: bool,
    pub show_nutritional_info: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformanceConfig {
    pub enable_template_cache: bool,
    pub enable_static_file_cache: bool,
    pub static_file_cache_max_age_seconds: u32,
    pub enable_rate_limiting: bool,
    pub rate_limit_requests_per_minute: u32,
    pub rate_limit_burst_size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DevelopmentConfig {
    pub hot_reload_templates: bool,
    pub expose_debug_endpoints: bool,
    pub verbose_error_messages: bool,
    pub enable_sql_query_logging: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeaturesConfig {
    pub enable_menu_presets: bool,
    pub enable_menu_scheduling: bool,
    pub enable_notices: bool,
    pub enable_allergen_tracking: bool,
    pub enable_dietary_filters: bool,
    pub enable_nutritional_info: bool,
    pub enable_multi_language: bool,
    pub enable_user_feedback: bool,
    pub enable_analytics: bool,
}

// Configuration Loading Functions
impl AdminConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        let config: AdminConfig = toml::from_str(&content)
            .map_err(|e| ConfigError::TomlParse(e.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), ConfigError> {
        if self.admin_users.is_empty() {
            return Err(ConfigError::Validation(
                "At least one admin user must be configured".to_string(),
            ));
        }
        
        // Check for duplicate usernames
        let mut usernames = std::collections::HashSet::new();
        for user in &self.admin_users {
            if !usernames.insert(&user.username) {
                return Err(ConfigError::Validation(format!(
                    "Duplicate username found: {}",
                    user.username
                )));
            }
        }
        
        Ok(())
    }
}

impl ValidationRules {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        let rules: ValidationRules = toml::from_str(&content)
            .map_err(|e| ConfigError::TomlParse(e.to_string()))?;
        Ok(rules)
    }
}

impl AppSettings {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        let settings: AppSettings = toml::from_str(&content)
            .map_err(|e| ConfigError::TomlParse(e.to_string()))?;
        Ok(settings)
    }
}
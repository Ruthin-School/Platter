use std::fs;
use std::io;
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

use crate::config::{AdminConfig, AppSettings, ValidationRules, ConfigError};
use crate::error_handler::AppError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

// Re-export types from original storage for compatibility
pub use crate::storage::{
    MenuItem, MenuCategory, Notice, MenuPreset, MenuSchedule,
    ScheduleRecurrence, ScheduleStatus
};

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(io::Error),
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Config error: {0}")]
    Config(#[from] ConfigError),
    #[error("RwLock poison error")]
    PoisonError,
    #[error("Permission denied: {0}. Please ensure the application has write access to the data directory.")]
    PermissionDenied(String),
    #[error("Validation error: {0}")]
    Validation(String),
}

impl From<io::Error> for StorageError {
    fn from(error: io::Error) -> Self {
        if error.kind() == io::ErrorKind::PermissionDenied {
            StorageError::PermissionDenied(error.to_string())
        } else {
            StorageError::Io(error)
        }
    }
}

impl From<StorageError> for AppError {
    fn from(storage_error: StorageError) -> Self {
        match storage_error {
            StorageError::Io(io_error) => AppError::Storage(io_error.to_string()),
            StorageError::Json(json_error) => AppError::Storage(json_error.to_string()),
            StorageError::Config(config_error) => AppError::Storage(config_error.to_string()),
            StorageError::PoisonError => AppError::Storage("RwLock poison error".to_string()),
            StorageError::PermissionDenied(msg) => AppError::Storage(msg),
            StorageError::Validation(msg) => AppError::Storage(msg),
        }
    }
}

// Enhanced JSON wrapper structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonDataFile<T> {
    pub schema_version: String,
    pub last_updated: String,
    pub generated_by: String,
    pub metadata: JsonMetadata,
    pub items: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_notices: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_schedules: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_presets: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_schedules: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<HashMap<String, usize>>,
    pub data_integrity_check: String,
}

impl<T> Default for JsonDataFile<T> {
    fn default() -> Self {
        Self {
            schema_version: "1.0.0".to_string(),
            last_updated: Utc::now().to_rfc3339(),
            generated_by: "platter-admin-ui".to_string(),
            metadata: JsonMetadata {
                total_items: None,
                active_notices: None,
                active_schedules: None,
                total_presets: None,
                total_schedules: None,
                categories: None,
                data_integrity_check: "passed".to_string(),
            },
            items: Vec::new(),
        }
    }
}

pub struct HybridStorage {
    // JSON data stores (enhanced with metadata)
    menu_items: Arc<RwLock<Vec<MenuItem>>>,
    notices: Arc<RwLock<Vec<Notice>>>,
    menu_presets: Arc<RwLock<Vec<MenuPreset>>>,
    menu_schedules: Arc<RwLock<Vec<MenuSchedule>>>,
    
    // Indexes for O(1) lookups
    menu_items_index: Arc<RwLock<HashMap<Uuid, usize>>>,
    presets_index: Arc<RwLock<HashMap<Uuid, usize>>>,
    schedules_index: Arc<RwLock<HashMap<Uuid, usize>>>,
    notices_index: Arc<RwLock<HashMap<Uuid, usize>>>,
    
    // TOML configuration
    admin_config: Arc<RwLock<AdminConfig>>,
    validation_rules: Arc<RwLock<ValidationRules>>,
    app_settings: Arc<RwLock<AppSettings>>,
    
    // File paths
    menu_items_path: String,
    notices_path: String,
    menu_presets_path: String,
    menu_schedules_path: String,
    admin_config_path: String,
    validation_rules_path: String,
    app_settings_path: String,
}

impl HybridStorage {
    pub fn new(
        data_dir: &str,
        config_dir: &str,
    ) -> Result<Self, StorageError> {
        log::info!("Initializing HybridStorage...");
        
        // Ensure directories exist
        fs::create_dir_all(data_dir)?;
        fs::create_dir_all(config_dir)?;
        
        // Define file paths
        let menu_items_path = format!("{}/menu_items.json", data_dir);
        let notices_path = format!("{}/notices.json", data_dir);
        let menu_presets_path = format!("{}/menu_presets.json", data_dir);
        let menu_schedules_path = format!("{}/menu_schedules.json", data_dir);
        let admin_config_path = format!("{}/admin.toml", config_dir);
        let validation_rules_path = format!("{}/validation.toml", config_dir);
        let app_settings_path = format!("{}/settings.toml", config_dir);
        
        // Load TOML configurations
        log::info!("Loading TOML configurations...");
        let admin_config = AdminConfig::load(&admin_config_path)?;
        let validation_rules = ValidationRules::load(&validation_rules_path)?;
        let app_settings = AppSettings::load(&app_settings_path)?;
        
        // Load JSON data
        log::info!("Loading JSON data...");
        let menu_items_data = Self::load_json_file::<MenuItem>(&menu_items_path)?;
        let notices_data = Self::load_json_file::<Notice>(&notices_path)?;
        let presets_data = Self::load_json_file::<MenuPreset>(&menu_presets_path)?;
        let schedules_data = Self::load_json_file::<MenuSchedule>(&menu_schedules_path)?;
        
        // Build indexes
        log::info!("Building indexes...");
        let menu_items_index = Self::build_index(&menu_items_data.items);
        let presets_index = Self::build_index(&presets_data.items);
        let schedules_index = Self::build_index(&schedules_data.items);
        let notices_index = Self::build_index(&notices_data.items);
        
        Ok(Self {
            menu_items: Arc::new(RwLock::new(menu_items_data.items)),
            notices: Arc::new(RwLock::new(notices_data.items)),
            menu_presets: Arc::new(RwLock::new(presets_data.items)),
            menu_schedules: Arc::new(RwLock::new(schedules_data.items)),
            menu_items_index: Arc::new(RwLock::new(menu_items_index)),
            presets_index: Arc::new(RwLock::new(presets_index)),
            schedules_index: Arc::new(RwLock::new(schedules_index)),
            notices_index: Arc::new(RwLock::new(notices_index)),
            admin_config: Arc::new(RwLock::new(admin_config)),
            validation_rules: Arc::new(RwLock::new(validation_rules)),
            app_settings: Arc::new(RwLock::new(app_settings)),
            menu_items_path,
            notices_path,
            menu_presets_path,
            menu_schedules_path,
            admin_config_path,
            validation_rules_path,
            app_settings_path,
        })
    }
    
    fn load_json_file<T: for<'de> Deserialize<'de> + Clone>(path: &str) -> Result<JsonDataFile<T>, StorageError> {
        if !Path::new(path).exists() {
            log::warn!("File {} not found, creating with empty data", path);
            let empty_file: JsonDataFile<T> = JsonDataFile::default();
            let json_data = serde_json::to_string_pretty(&empty_file)?;
            fs::write(path, json_data)?;
            return Ok(empty_file);
        }
        
        let content = fs::read_to_string(path)?;
        let data: JsonDataFile<T> = serde_json::from_str(&content)?;
        Ok(data)
    }
    
    fn build_index<T>(items: &[T]) -> HashMap<Uuid, usize> 
    where
        T: HasId,
    {
        items.iter()
            .enumerate()
            .map(|(idx, item)| (item.get_id(), idx))
            .collect()
    }
    
    fn save_json_file<T: Serialize + Clone>(
        path: &str,
        items: &[T],
        metadata_fn: impl FnOnce(&[T]) -> JsonMetadata,
    ) -> Result<(), StorageError> {
        let data_file = JsonDataFile {
            schema_version: "1.0.0".to_string(),
            last_updated: Utc::now().to_rfc3339(),
            generated_by: "platter-admin-ui".to_string(),
            metadata: metadata_fn(items),
            items: items.to_vec(),
        };
        
        let json_data = serde_json::to_string_pretty(&data_file)?;
        fs::write(path, json_data)?;
        Ok(())
    }
    
    // Public getters
    pub fn get_menu_items(&self) -> Result<Vec<MenuItem>, StorageError> {
        let items = self.menu_items.read()
            .map_err(|_| StorageError::PoisonError)?;
        Ok(items.clone())
    }
    
    pub fn get_admin_config(&self) -> Result<AdminConfig, StorageError> {
        let config = self.admin_config.read()
            .map_err(|_| StorageError::PoisonError)?;
        Ok(config.clone())
    }
    
    pub fn get_validation_rules(&self) -> Result<ValidationRules, StorageError> {
        let rules = self.validation_rules.read()
            .map_err(|_| StorageError::PoisonError)?;
        Ok(rules.clone())
    }
    
    pub fn get_app_settings(&self) -> Result<AppSettings, StorageError> {
        let settings = self.app_settings.read()
            .map_err(|_| StorageError::PoisonError)?;
        Ok(settings.clone())
    }
}

// Helper trait for items with IDs
trait HasId {
    fn get_id(&self) -> Uuid;
}

impl HasId for MenuItem {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl HasId for Notice {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl HasId for MenuPreset {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl HasId for MenuSchedule {
    fn get_id(&self) -> Uuid {
        self.id
    }
}
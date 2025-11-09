use serde::Deserialize;

mod typing;
pub use typing::{
    FaultDomainAwareness, HealthStatus, OperationalStatus, ProvisioningType, ReadOnlyReason,
    RepairPolicy, RetireMissingPhysicalDisks, Usage, Version,
};

/// Represents a logical grouping of physical disks that may be used to create virtual disks.
#[derive(Debug, Deserialize)]
#[serde(rename = "MSFT_StoragePool")]
#[serde(rename_all = "PascalCase")]
pub struct StoragePool {
    /// Object Path for associated queries, method call.
    #[serde(rename = "__Path")]
    pub obj_path: String,

    /// ObjectId is a mandatory property that is used to opaquely and uniquely identify
    /// an instance of a class. ObjectId values are required to be globally unique.
    ///
    /// That is, no two objects should ever have the same ObjectId,
    /// even if they are managed by separate storage management providers,
    /// or are on different storage subsystems.
    pub object_id: String,

    /// UniqueId is a mandatory property that is used to uniquely identify a logical
    /// instance of a storage subsystem's object.
    ///
    /// This value must be the same for an object viewed by two or more provider instances,
    /// even if they are running on separate management servers.
    pub unique_id: String,

    /// A user-friendly name for the storage pool
    pub friendly_name: Option<String>,

    /// Semi-unique human-readable identifier scoped to the storage subsystem
    pub name: Option<String>,

    /// Intended usage of the pool
    pub usage: Usage,

    /// Custom usage description when Usage is Other
    pub other_usage_description: Option<String>,

    /// TRUE if this is the primordial ("available storage") pool
    pub is_primordial: bool,

    /// Health status derived from physical disks and redundancy
    pub health_status: HealthStatus,

    /// Current operational status (can contain multiple values)
    pub operational_status: Vec<OperationalStatus>,

    /// Vendor-specific status when OperationalStatus contains Other
    pub other_operational_status_description: Option<String>,

    /// Total capacity of the pool in bytes
    pub size: Option<u64>,

    /// Total allocated capacity in bytes
    pub allocated_size: Option<u64>,

    /// Logical sector size in bytes
    pub logical_sector_size: Option<u64>,

    /// Physical sector size in bytes
    pub physical_sector_size: Option<u64>,

    /// Default provisioning type for new virtual disks
    pub provisioning_type_default: ProvisioningType,

    /// Supported provisioning types
    pub supported_provisioning_types: Vec<ProvisioningType>,

    /// Default resiliency setting name (corresponds to MSFT_ResiliencySetting.Name)
    pub resiliency_setting_name_default: Option<String>,

    /// TRUE if configuration is locked
    pub is_read_only: Option<bool>,

    /// Reason for read-only state
    pub read_only_reason: Option<ReadOnlyReason>,

    /// TRUE if used in a failover cluster
    pub is_clustered: Option<bool>,
    /// TRUE if deduplication is supported
    pub supports_deduplication: Option<bool>,

    /// Thin-provisioning alert thresholds (percentages)
    pub thin_provisioning_alert_thresholds: Option<Vec<u16>>,

    /// TRUE if disks are zeroed on unmap/remove
    pub clear_on_deallocate: Option<bool>,

    /// TRUE if disks preserve data after power loss
    pub is_power_protected: Option<bool>,

    /// Repair policy for virtual disks
    pub repair_policy: Option<RepairPolicy>,

    /// Default enclosure-aware placement for new virtual disks
    pub enclosure_aware_default: Option<bool>,

    /// Default fault domain awareness level
    pub fault_domain_awareness_default: Option<FaultDomainAwareness>,

    /// Policy for retiring missing physical disks
    pub retire_missing_physical_disks: Option<RetireMissingPhysicalDisks>,

    /// Minimum OS version that supports this pool
    pub version: Option<Version>,
    /// Default write-cache size for new virtual disks
    pub write_cache_size_default: Option<u64>,

    /// Minimum allowed write-cache size
    pub write_cache_size_min: Option<u64>,

    /// Maximum allowed write-cache size
    pub write_cache_size_max: Option<u64>,
}

// Empty association structs (required for WMI object graph navigation)

#[derive(Debug, Deserialize)]
#[serde(rename = "MSFT_StoragePoolToPhysicalDisk")]
pub struct StoragePoolToPhysicalDisk;

#[derive(Debug, Deserialize)]
#[serde(rename = "MSFT_StoragePoolToResiliencySetting")]
pub struct StoragePoolToResiliencySetting;

#[derive(Debug, Deserialize)]
#[serde(rename = "MSFT_StoragePoolToStorageTier")]
pub struct StoragePoolToStorageTier;

#[derive(Debug, Deserialize)]
#[serde(rename = "MSFT_StoragePoolToVirtualDisk")]
pub struct StoragePoolToVirtualDisk;

#[derive(Debug, Deserialize)]
#[serde(rename = "MSFT_StoragePoolToVolume")]
pub struct StoragePoolToVolume;

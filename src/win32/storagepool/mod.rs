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

    /// The identifier for the physical disk that is persistent across reboots.
    ///
    /// This is typically a number (e.g., "0", "1") assigned by the storage subsystem.
    pub device_id: String,

    /// ObjectId is a mandatory property that is used to opaquely and uniquely identify
    /// an instance of a class. ObjectId values are required to be globally unique.
    ///
    /// That is, no two objects should ever have the same ObjectId,
    /// even if they are managed by separate storage management providers,
    /// or are on different storage subsystems.
    pub object_id: String,

    /// A user-friendly name for the storage pool
    pub friendly_name: String,

    /// Semi-unique human-readable identifier scoped to the storage subsystem
    pub name: String,

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
    pub size: u64,

    /// Total allocated capacity in bytes
    pub allocated_size: u64,

    /// Logical sector size in bytes
    pub logical_sector_size: u64,

    /// Physical sector size in bytes
    pub physical_sector_size: u64,

    /// Default provisioning type for new virtual disks
    pub provisioning_type_default: ProvisioningType,

    /// Supported provisioning types
    pub supported_provisioning_types: Vec<ProvisioningType>,

    /// Default resiliency setting name (corresponds to MSFT_ResiliencySetting.Name)
    pub resiliency_setting_name_default: String,

    /// TRUE if configuration is locked
    pub is_read_only: bool,

    /// Reason for read-only state
    pub read_only_reason: ReadOnlyReason,

    /// TRUE if used in a failover cluster
    pub is_clustered: bool,

    /// TRUE if deduplication is supported
    pub supports_deduplication: bool,

    /// Thin-provisioning alert thresholds (percentages)
    pub thin_provisioning_alert_thresholds: Vec<u16>,

    /// TRUE if disks are zeroed on unmap/remove
    pub clear_on_deallocate: bool,

    /// TRUE if disks preserve data after power loss
    pub is_power_protected: bool,

    /// Repair policy for virtual disks
    pub repair_policy: RepairPolicy,

    /// Default enclosure-aware placement for new virtual disks
    pub enclosure_aware_default: bool,

    /// Default fault domain awareness level
    #[serde(rename = "FaultDomainAwarenessDefault")]
    pub fault_domain_awareness_default: FaultDomainAwareness,

    /// Policy for retiring missing physical disks
    pub retire_missing_physical_disks: RetireMissingPhysicalDisks,

    /// Minimum OS version that supports this pool
    pub version: Version,

    /// Default write-cache size for new virtual disks
    pub write_cache_size_default: u64,

    /// Minimum allowed write-cache size
    pub write_cache_size_min: u64,

    /// Maximum allowed write-cache size
    pub write_cache_size_max: u64,
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

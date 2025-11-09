use serde_repr::Deserialize_repr;

/// Health status of the storage pool
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr)]
#[repr(u16)]
pub enum HealthStatus {
    Healthy = 0,
    Warning = 1,
    Unhealthy = 2,
    Unknown = 5,
}

/// Operational status values for the storage pool (can have multiple)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr)]
#[repr(u16)]
pub enum OperationalStatus {
    Unknown = 0,
    Other = 1,
    OK = 2,
    Degraded = 3,
    Stressed = 4,
    PredictiveFailure = 5,
    Error = 6,
    NonRecoverableError = 7,
    Starting = 8,
    Stopping = 9,
    Stopped = 10,
    InService = 11,
    NoContact = 12,
    LostCommunication = 13,
    Aborted = 14,
    Dormant = 15,
    SupportingEntityInError = 16,
    Completed = 17,
    PowerMode = 18,
    Relocating = 19,
    MajorityDisksUnhealthy = 0x8000,
    MinorityDisksUnhealthy = 0x8001,
}

/// Default provisioning type for new virtual disks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr)]
#[repr(u16)]
pub enum ProvisioningType {
    Unknown = 0,
    Thin = 1,
    Fixed = 2,
}

/// Fault domain awareness level (default for new virtual disks)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr)]
#[repr(u16)]
pub enum FaultDomainAwareness {
    PhysicalDisk = 1,
    StorageEnclosure = 2,
    StorageScaleUnit = 3,
    StorageChassis = 4,
    StorageRack = 5,
}

/// Reason why the pool is read-only
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr)]
#[repr(u16)]
pub enum ReadOnlyReason {
    Unknown = 0,
    None = 1,
    ByPolicy = 2,
    MajorityDisksUnhealthy = 3,
}

/// Repair policy for virtual disks in this pool
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr)]
#[repr(u16)]
pub enum RepairPolicy {
    Sequential = 2,
    Parallel = 3,
}

/// Policy for retiring missing physical disks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr)]
#[repr(u16)]
pub enum RetireMissingPhysicalDisks {
    Auto = 1,
    Always = 2,
    Never = 3,
}

/// Intended usage of the storage pool
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr)]
#[repr(u16)]
pub enum Usage {
    Unknown = 0,
    Other = 1,
    Unrestricted = 2,
    ReservedForComputerSystem = 3,
    ReservedAsDeltaReplicaContainer = 4,
    ReservedForMigrationServices = 5,
    ReservedForLocalReplicationServices = 6,
    ReservedForRemoteReplicationServices = 7,
    ReservedForSparing = 8,
}

/// Minimum supported OS version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr)]
#[repr(u16)]
pub enum Version {
    WindowsServer2012 = 1,
    WindowsServer2012R2Preview = 2,
    WindowsServer2012R2 = 3,
    WindowsServer2016Preview0 = 4,
    WindowsServer2016Preview1 = 5,
    WindowsServer2016Preview2 = 6,
    WindowsServer2016Preview3 = 7,
    WindowsServer2016Preview4 = 8,
    WindowsServer2016Preview5 = 9,
    WindowsServer2016Preview6 = 10,
    WindowsServer2016Preview7 = 11,
    WindowsServer2016Preview8 = 12,
    WindowsServer2016Preview9 = 13,
    WindowsServer2016PreviewA = 14,
    WindowsServer2016PreviewB = 15,
    WindowsServer2016PreviewC = 16,
    WindowsServer2016PreviewD = 17,
    WindowsServer2016PreviewE = 18,
    WindowsServer2016 = 19,
    WindowsServer2016RS3 = 20,
    WindowsServer2019Preview = 21,
    WindowsServer2019 = 22,
    WindowsServer2022Preview1 = 23,
    WindowsServer2022Preview2 = 24,
    WindowsServer2022Preview3 = 25,
    WindowsServer2022Preview4 = 26,
    WindowsServer2022 = 27,
    WindowsServer2025 = 28,
    WindowsServer2025SP1 = 29,
}

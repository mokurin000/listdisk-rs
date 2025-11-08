use serde_repr::{Deserialize_repr, Serialize_repr};

/// `Availability` property values (uint16)
#[derive(Deserialize_repr, Serialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum Availability {
    Other = 1,
    Unknown = 2,
    RunningFullPower = 3,
    Warning = 4,
    InTest = 5,
    NotApplicable = 6,
    PowerOff = 7,
    OffLine = 8,
    OffDuty = 9,
    Degraded = 10,
    NotInstalled = 11,
    InstallError = 12,
    PowerSaveUnknown = 13,
    PowerSaveLowPowerMode = 14,
    PowerSaveStandby = 15,
    PowerCycle = 16,
    PowerSaveWarning = 17,
    Paused = 18,
    NotReady = 19,
    NotConfigured = 20,
    Quiesced = 21,
}

/// `StatusInfo` property values (uint16)
#[derive(Deserialize_repr, Serialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum StatusInfo {
    Other = 1,
    Unknown = 2,
    Enabled = 3,
    Disabled = 4,
    NotApplicable = 5,
}

/// `Capabilities` array values (uint16)
#[derive(Deserialize_repr, Serialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum Capability {
    Unknown = 0,
    Other = 1,
    SequentialAccess = 2,
    RandomAccess = 3,
    SupportsWriting = 4,
    Encryption = 5,
    Compression = 6,
    SupportsRemovableMedia = 7,
    ManualCleaning = 8,
    AutomaticCleaning = 9,
    SmartNotification = 10,
    SupportsDualSidedMedia = 11,
    EjectPriorToDismountNotRequired = 12,
}

/// `PowerManagementCapabilities` array values (uint16)
#[derive(Deserialize_repr, Serialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum PowerManagementCapability {
    Unknown = 0,
    NotSupported = 1,
    Disabled = 2,
    Enabled = 3,
    PowerSavingModesEnteredAutomatically = 4,
    PowerStateSettable = 5,
    PowerCyclingSupported = 6,
    TimedPowerOnSupported = 7,
}

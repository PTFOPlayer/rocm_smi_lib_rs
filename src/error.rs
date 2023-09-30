#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum RocmErr {
    RsmiStatusSuccess,
    RsmiStatusInvalidArgs,
    RsmiStatusNotSupported,
    RsmiStatusFileError,
    RsmiStatusPermission,
    RsmiStatusOutOfResources,
    RsmiStatusInternalException,
    RsmiStatusInputOutOfBounds,
    RsmiStatusInitError,
    RsmiStatusNotYetImplemented,
    RsmiStatusNotFound,
    RsmiStatusInsufficientSize,
    RsmiStatusInterrupt,
    RsmiStatusUnexpectedSize,
    RsmiStatusNoData,
    RsmiStatusUnexpectedData,
    RsmiStatusBusy,
    RsmiStatusRefcountOverflow,
    RsmiStatusSettingUnavailable,
    RsmiStatusAmdgpuRestartErr,
    RsmiStringConversionError,
    RsmiStatusUnknownError = 0xFFFFFFFF,
}

impl RocmErr {
    pub fn from_u16(code: u16) -> Self {
        match code {
            0 => RocmErr::RsmiStatusSuccess,
            1 => RocmErr::RsmiStatusInvalidArgs,
            2 => RocmErr::RsmiStatusNotSupported,
            3 => RocmErr::RsmiStatusFileError,
            4 => RocmErr::RsmiStatusPermission,
            5 => RocmErr::RsmiStatusOutOfResources,
            6 => RocmErr::RsmiStatusInternalException,
            7 => RocmErr::RsmiStatusInputOutOfBounds,
            8 => RocmErr::RsmiStatusInitError,
            9 => RocmErr::RsmiStatusNotYetImplemented,
            10 => RocmErr::RsmiStatusNotFound,
            11 => RocmErr::RsmiStatusInsufficientSize,
            12 => RocmErr::RsmiStatusInterrupt,
            13 => RocmErr::RsmiStatusUnexpectedSize,
            14 => RocmErr::RsmiStatusNoData,
            15 => RocmErr::RsmiStatusUnexpectedData,
            16 => RocmErr::RsmiStatusBusy,
            17 => RocmErr::RsmiStatusRefcountOverflow,
            18 => RocmErr::RsmiStatusSettingUnavailable,
            19 => RocmErr::RsmiStatusAmdgpuRestartErr,
            20 => RocmErr::RsmiStatusUnknownError,
            21 => RocmErr::RsmiStringConversionError,
            _ => RocmErr::RsmiStatusUnknownError,
        }
    }

    pub fn into_u16(self) -> u16 {
        self as u16
    }

    pub fn try_err(self) -> Result<(), Self> {
        match self {
            RocmErr::RsmiStatusSuccess => Ok(()),
            _ => Err(self),
        }
    }
}

impl ToString for RocmErr {
    fn to_string(&self) -> String {
        match self {
            RocmErr::RsmiStatusSuccess => "Rsmi status success".to_owned(),
            RocmErr::RsmiStatusInvalidArgs => "Rsmi status invalid arguments".to_owned(),
            RocmErr::RsmiStatusNotSupported => "Rsmi status not supported (device or function)".to_owned(),
            RocmErr::RsmiStatusFileError => "Rsmi status file error".to_owned(),
            RocmErr::RsmiStatusPermission => "Rsmi status permission (use sudo)".to_owned(),
            RocmErr::RsmiStatusOutOfResources => "Rsmi status out of resources".to_owned(),
            RocmErr::RsmiStatusInternalException => "Rsmi status internal exception".to_owned(),
            RocmErr::RsmiStatusInputOutOfBounds => "Rsmi status input out of bounds".to_owned(),
            RocmErr::RsmiStatusInitError => "Rsmi status init error".to_owned(),
            RocmErr::RsmiStatusNotYetImplemented => "Rsmi status not yet implemented".to_owned(),
            RocmErr::RsmiStatusNotFound => "Rsmi status not found".to_owned(),
            RocmErr::RsmiStatusInsufficientSize => "Rsmi status insufficient size (sorry you can not fix that on your own, create issue on github with info about your GPU)".to_owned(),
            RocmErr::RsmiStatusInterrupt => "Rsmi status interrupt".to_owned(),
            RocmErr::RsmiStatusUnexpectedSize => "Rsmi status unexpected size (sorry you can not fix that on your own, create issue on github with info about your GPU)".to_owned(),
            RocmErr::RsmiStatusNoData => "Rsmi status no data (mostly when your gpu supports function but does not return data)".to_owned(),
            RocmErr::RsmiStatusUnexpectedData => "Rsmi status unexpected data".to_owned(),
            RocmErr::RsmiStatusBusy => "Rsmi status busy (just use sleep after previous function)".to_owned(),
            RocmErr::RsmiStatusRefcountOverflow => "Rsmi status refcount overflow".to_owned(),
            RocmErr::RsmiStatusSettingUnavailable => "Rsmi status setting unavailable".to_owned(),
            RocmErr::RsmiStatusAmdgpuRestartErr => "Rsmi status Amdgpu restart err (you probably need to reboot or load gpu driver again".to_owned(),
            RocmErr::RsmiStatusUnknownError => "Rsmi status unknown error".to_owned(),
            RocmErr::RsmiStringConversionError => "Rsmi status string conversion error (sorry you can not fix that on your own, create issue on github with info about your GPU)".to_owned(),
        }
    }
}

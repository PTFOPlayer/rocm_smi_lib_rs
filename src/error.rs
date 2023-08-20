#[derive(Debug)]
pub enum RocmErr {
    RsmiStatusSuccess = 0,
    RsmiStatusInvalidArgs = 1,
    RsmiStatusNotSupported = 2,
    RsmiStatusFileError = 3,
    RsmiStatusPermission = 4,
    RsmiStatusOutOfResources = 5,
    RsmiStatusInternalException = 6,
    RsmiStatusInputOutOfBounds = 7,
    RsmiStatusInitError = 8,
    RsmiStatusNotYetImplemented = 9,
    RsmiStatusNotFound = 10,
    RsmiStatusInsufficientSize = 11,
    RsmiStatusInterrupt = 12,
    RsmiStatusUnexpectedSize = 13,
    RsmiStatusNoData = 14,
    RsmiStatusUnexpectedData = 15,
    RsmiStatusBusy = 16,
    RsmiStatusRefcountOverflow = 17,
    RsmiStatusSettingUnavailable = 18,
    RsmiStatusAmdgpuRestartErr = 19,
    RsmiStatusUnknownError = 20,
    RsmiStringConversionError = 21,
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
        match self {
            RocmErr::RsmiStatusSuccess => 0 ,
            RocmErr::RsmiStatusInvalidArgs => 1 ,
            RocmErr::RsmiStatusNotSupported => 2 ,
            RocmErr::RsmiStatusFileError => 3 ,
            RocmErr::RsmiStatusPermission => 4 ,
            RocmErr::RsmiStatusOutOfResources => 5 ,
            RocmErr::RsmiStatusInternalException => 6 ,
            RocmErr::RsmiStatusInputOutOfBounds => 7 ,
            RocmErr::RsmiStatusInitError => 8 ,
            RocmErr::RsmiStatusNotYetImplemented => 9 ,
            RocmErr::RsmiStatusNotFound => 10 ,
            RocmErr::RsmiStatusInsufficientSize => 11 ,
            RocmErr::RsmiStatusInterrupt => 12 ,
            RocmErr::RsmiStatusUnexpectedSize => 13 ,
            RocmErr::RsmiStatusNoData => 14 ,
            RocmErr::RsmiStatusUnexpectedData => 15 ,
            RocmErr::RsmiStatusBusy => 16 ,
            RocmErr::RsmiStatusRefcountOverflow => 17 ,
            RocmErr::RsmiStatusSettingUnavailable => 18 ,
            RocmErr::RsmiStatusAmdgpuRestartErr => 19 ,
            RocmErr::RsmiStatusUnknownError => 20,
            RocmErr::RsmiStringConversionError => 21
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
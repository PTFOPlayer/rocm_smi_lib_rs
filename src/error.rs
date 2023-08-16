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
            RocmErr::RsmiStatusUnknownError => 20
        }
    }
}

use libc::{c_int, c_char};

#[repr(C)]
struct result_str {
    status: c_int,
    data: *mut c_char
}

#[repr(C)]
struct result_int {
    status: c_int,
    data: c_int
}

#[link(name = "rsmi64", kind = "static")]
extern "C" {
    fn num_devices() -> result_int;
    fn init_c() -> i32;
}

#[derive(Debug)]
enum RocmErr {
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
    pub fn from_i32(code:i32) -> Self {
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
            _ => panic!("unknown error occured in ROCM_SMI")
        }
    }
}

struct RocmSmi {}

impl RocmSmi {
    pub fn init() -> Result<Self, RocmErr> {
        let code = unsafe {  
            init_c()
        };
        if code == 0 {
            return Ok(RocmSmi {});
        }
        Err(RocmErr::from_i32(code))
    }

    pub fn device_count(&self) -> Result<u32, RocmErr> {
        let res = unsafe {
            num_devices()
        };

        if res.status != 0 {
            return Err(RocmErr::from_i32(res.status));
        }

        Ok(res.data as u32)
    }
    
}


fn main() {
    match RocmSmi::init() {
        Ok(res) => {
            println!("{:?}", res.device_count());
        },
        Err(err) => println!("{:?}", err),
    }
}

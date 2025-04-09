// currently not available

// use std::mem::size_of;

// use rocm_smi_lib_sys::bindings::*;

// use crate::error::RocmErr;
// pub unsafe fn get_supported_fn() -> Result<Vec<String>, RocmErr> {
//     let mut handle = RsmiFuncIdIterHandle::new();
//     let hdl_ptr = &mut handle as *mut RsmiFuncIdIterHandle;
//     raw.rsmi_dev_supported_func_iterator_open(0, hdl_ptr)
//         .try_err()?;

//     let mut value = RsmiFuncIdValue::default();
//     let val_ptr = &mut value as *mut RsmiFuncIdValue;

//     let mut names = vec![];

//     loop {
//         raw.rsmi_func_iter_value_get(handle, val_ptr).try_err()?;
//         let buff = libc::malloc(128 * size_of::<i8>()).cast::<i8>();
//         value.name.cast::<i8>().copy_to_nonoverlapping(buff, 128);
//         let temp = std::ffi::CString::from_raw(buff);
//         let mut fn_name = temp.to_string_lossy().to_string();
//         fn_name.shrink_to_fit();
//         names.push(fn_name);

//         let res = raw.rsmi_func_iter_next(handle);
//         if res == RocmErr::RsmiStatusNoData {
//             break;
//         }
//     }

//     Ok(names)
// }

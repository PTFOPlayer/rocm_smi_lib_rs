use rocm_smi_lib_sys::bindings::{
    rsmi_dev_supported_func_iterator_close, rsmi_dev_supported_func_iterator_open,
    rsmi_dev_supported_variant_iterator_open, rsmi_func_id_iter_handle_t, rsmi_func_id_value_t,
    rsmi_func_iter_next, rsmi_func_iter_value_get, rsmi_status_t_RSMI_STATUS_NO_DATA,
};
use std::ffi::CStr;
use std::ptr;

use crate::{IntoRocmErr, RocmErr, RocmSmi};

#[derive(Debug)]
pub struct Variant {
    pub id: u32,
    pub sub_variants: Vec<u32>,
}

#[derive(Debug)]
pub struct FunctionInfo {
    pub name: String,
    pub variants: Vec<Variant>,
}
impl RocmSmi {
    pub fn get_supported_fn(&self, dev_id: u32) -> Result<Vec<FunctionInfo>, RocmErr> {
        let mut result = Vec::new();

        let mut iter_handle: rsmi_func_id_iter_handle_t = ptr::null_mut();
        unsafe { rsmi_dev_supported_func_iterator_open(dev_id, &mut iter_handle) }
            .into_rocm_err()?;

        loop {
            let mut value: rsmi_func_id_value_t = unsafe { std::mem::zeroed() };
            let err = unsafe { rsmi_func_iter_value_get(iter_handle, &mut value) };

            if err == rsmi_status_t_RSMI_STATUS_NO_DATA {
                break;
            }
            err.into_rocm_err()?;

            let name = unsafe { CStr::from_ptr(value.name).to_string_lossy().to_string() };
            let mut variants = Vec::new();

            let mut var_iter: rsmi_func_id_iter_handle_t = ptr::null_mut();
            let err =
                unsafe { rsmi_dev_supported_variant_iterator_open(iter_handle, &mut var_iter) };

            if err != rsmi_status_t_RSMI_STATUS_NO_DATA {
                err.into_rocm_err()?;

                loop {
                    let mut var_val: rsmi_func_id_value_t = unsafe { std::mem::zeroed() };
                    let err = unsafe { rsmi_func_iter_value_get(var_iter, &mut var_val) };
                    if err == rsmi_status_t_RSMI_STATUS_NO_DATA {
                        break;
                    }
                    err.into_rocm_err()?;

                    let mut sub_var_ids = Vec::new();
                    let mut sub_var_iter: rsmi_func_id_iter_handle_t = ptr::null_mut();

                    let err = unsafe {
                        rsmi_dev_supported_variant_iterator_open(var_iter, &mut sub_var_iter)
                    };
                    if err != rsmi_status_t_RSMI_STATUS_NO_DATA {
                        err.into_rocm_err()?;

                        loop {
                            let mut sub_val: rsmi_func_id_value_t = unsafe { std::mem::zeroed() };
                            let err =
                                unsafe { rsmi_func_iter_value_get(sub_var_iter, &mut sub_val) };
                            if err == rsmi_status_t_RSMI_STATUS_NO_DATA {
                                break;
                            }
                            err.into_rocm_err()?;

                            sub_var_ids.push(unsafe { sub_val.id } as u32);

                            let err = unsafe { rsmi_func_iter_next(sub_var_iter) };
                            if err == rsmi_status_t_RSMI_STATUS_NO_DATA {
                                break;
                            }
                            err.into_rocm_err()?;
                        }

                        unsafe {
                            rsmi_dev_supported_func_iterator_close(&mut sub_var_iter);
                        }
                    }

                    variants.push(Variant {
                        id: unsafe { var_val.id as u32 },
                        sub_variants: sub_var_ids,
                    });

                    let err = unsafe { rsmi_func_iter_next(var_iter) };
                    if err == rsmi_status_t_RSMI_STATUS_NO_DATA {
                        break;
                    }
                    err.into_rocm_err()?;
                }

                unsafe {
                    rsmi_dev_supported_func_iterator_close(&mut var_iter);
                }
            }

            result.push(FunctionInfo { name, variants });

            let err = unsafe { rsmi_func_iter_next(iter_handle) };
            if err == rsmi_status_t_RSMI_STATUS_NO_DATA {
                break;
            }
            err.into_rocm_err()?;
        }

        unsafe {
            rsmi_dev_supported_func_iterator_close(&mut iter_handle);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use crate::{RocmErr, RocmSmi};

    #[test]
    fn get_supported_fn_test() -> Result<(), RocmErr> {
        println!("{:#?}", RocmSmi::init()?.get_supported_fn(0)?);

        Ok(())
    }
}

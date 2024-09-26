# rocm_smi_lib_rs
Crate bringing support for rocm-smi liblary from C

# Compatibility 
| Crate version | Rocm version  |
| ------------- | ------------- |
| <2.0          | <6.0          |
| 2.1 - 2.2     | PROVEN BROKEN |
| 2.3           | 6.0-6.2       |

# Requirements

## To use this crate you have to install rocm-smi liblaries
Check out [How to install ROCM](https://rocm.docs.amd.com/projects/install-on-linux/en/latest/how-to/native-install/index.html)

# Usage / Examples

Example of printing the name of your GPU:
```rust
fn print_gpu_name() -> Result<(), RocmErr> {
    let rocm = RocmSmi::init()?;
    let name = rocm.get_device_identifiers(0)?.name;
    println!("{}", name);
    Ok(())
}    
```

Same thing as above but with creation of device object
```rust
fn print_gpu_name() -> Result<(), RocmErr> {
    let rocm = RocmSmi::init()?.into_first_device()?; // into first devic requires enabling `device` feature
    let name = rocm.get_identifiers()?.name;
    println!("{}", name);
    Ok(())
}    
```

# RC versions
Please consider testing rc versions and report bugs in them, api is large and it takes a lot of time to test everything alone on many GPU's.

# TODO
- [ ] setters (it will not be done until i have som testing crew and/or figure out automatic tests across different gpu's)

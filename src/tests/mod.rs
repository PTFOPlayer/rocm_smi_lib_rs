#[cfg(test)]


mod test {
    use std::time::Duration;

    use rocm_smi_lib_sys::{
        bindings::{RsmiClkType, RsmiTemperatureMetric, RsmiTemperatureSensor, RsmiVoltageMetric},
        error::RocmErr,
    };

    use crate::RocmSmi ;

    #[test]
    #[cfg(feature = "device")]
    fn identifers_test() -> Result<(), RocmErr> {
        let mut res = RocmSmi::init()?.into_first_device()?;
        let identifiers = res.get_identifiers()?;
        println!("identifiers: {:?}", identifiers);
        Ok(())
    }

    #[test]
    #[cfg(feature = "device")]
    fn full_metrics_test() -> Result<(), RocmErr> {
        let mut res = RocmSmi::init()?.into_first_device()?;
        let metrics = res.get_full_metrics()?;
        println!("metrics: {:?}", metrics);
        Ok(())
    }


    #[test]
    #[cfg(feature = "device")]
    fn pcie_test() -> Result<(), RocmErr> {
        let mut res = RocmSmi::init()?.into_first_device()?;
        println!("pcie data: {:?}", res.get_pcie_data());
        Ok(())
    }
    #[test]
    #[cfg(feature = "device")]
    fn pwr_test() -> Result<(), RocmErr> {
        let mut res = RocmSmi::init()?.into_first_device()?;
        println!("power data: {:?}", res.get_power_data());
        Ok(())
    }
    #[test]
    #[cfg(feature = "device")]
    fn mem_test() -> Result<(), RocmErr> {
        let mut res = RocmSmi::init()?.into_first_device()?;
        println!("memory data: {:?}", res.get_memory_data());
        Ok(())
    }
    #[test]
    #[cfg(feature = "device")]
    fn fans_test() -> Result<(), RocmErr> {
        let mut res = RocmSmi::init()?.into_first_device()?;
        println!("fans data: {:?}", res.get_fans_data());
        Ok(())
    }
    #[test]
    #[cfg(feature = "device")]
    fn junction_temp_test() -> Result<(), RocmErr> {
        let mut res = RocmSmi::init()?.into_first_device()?;
        println!(
            "junction temperature data: {:?}",
            res.get_temperature_metric(
                RsmiTemperatureSensor::RsmiTempTypeJunction,
                RsmiTemperatureMetric::RsmiTempCurrent
            )
        );
        Ok(())
    }

    #[test]
    #[cfg(feature = "device")]
    fn mem_temp_test() -> Result<(), RocmErr> {
        let mut res = RocmSmi::init()?.into_first_device()?;
        println!(
            "memory temperature data: {:?}",
            res.get_temperature_metric(
                RsmiTemperatureSensor::RsmiTempTypeMemory,
                RsmiTemperatureMetric::RsmiTempCurrent,
            )
        );
        Ok(())
    }

    #[test]
    #[cfg(feature = "fn_query")]
    fn supported_fn_test() -> Result<(), RocmErr> {
        let mut res = RocmSmi::init()?;
        println!("supported functions:");
        let names = res.get_supported_functions()?;
        for name in names  {
            println!("\t{:?}", name);
        }
        Ok(())
    }

    #[cfg(feature = "device")]
    #[test]
    fn voltage_test() -> Result<(), RocmErr> {
        let mut res = RocmSmi::init()?.into_first_device()?;
        println!("{}", res.id);
        println!(
            "voltage data: {:?}",
            res.get_voltage_metric(RsmiVoltageMetric::RsmiVoltCurrent)
        );
        Ok(())
    }

    #[cfg(feature = "device")]
    #[test]
    fn main_test() -> Result<(), RocmErr> {
        let mut res = RocmSmi::init()?.into_first_device()?;
        println!("{}", res.id);
        std::thread::sleep(Duration::from_secs_f32(2.));
        println!("busy percent: {:?}", res.get_busy_percent());
        println!("perf counters: {:?}", res.get_performance_countes());
        println!("perf level: {:?}", res.get_performance_level());
        println!("overdrive level: {:?}", res.get_overdrive_levels());
        println!(
            "freq core {:?}",
            res.get_frequency(RsmiClkType::RsmiClkTypeDf)
        );
        println!("f-v curve: {:?}", res.get_frequency_voltage_curve());
        // until further fixes in sys liblary
        // println!("metrics: {:?}", res.get_full_metrics());
        println!("ecc: {:?}", res.get_ecc_data());
        println!("vbios: {:?}", res.get_vbios_version());


        println!("rsmi_v: {:?}", res.get_rsmi_version());
        Ok(())
    }
}
#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::{
        bindings::{RsmiClkType, RsmiTemperatureMetric, RsmiTemperatureSensor, RsmiVoltageMetric},
        error::RocmErr,
        RocmSmi,
    };

    #[test]
    fn full_test() -> Result<(), RocmErr> {
        let res = RocmSmi::init()?.into_first_device()?;

        let identifiers = res.get_identifiers()?;
        std::thread::sleep(Duration::from_secs_f32(0.5));
        println!("identifiers: {:?}", identifiers);
        println!(
            "unique id (might fail if there is only one gpu) {:?}",
            identifiers.get_unique_id()
        );
        println!("pcie data: {:?}", res.get_pcie_data());
        println!("power data: {:?}", res.get_power_data());
        println!("memory data: {:?}", res.get_memory_data());
        println!("fans data: {:?}", res.get_fans_data());
        println!(
            "junction temperature data: {:?}",
            res.get_temperature_metric(
                RsmiTemperatureSensor::RsmiTempTypeJunction,
                RsmiTemperatureMetric::RsmiTempCurrent
            )
        );
        println!(
            "memory temperature data: {:?}",
            res.get_temperature_metric(
                RsmiTemperatureSensor::RsmiTempTypeEdge,
                RsmiTemperatureMetric::RsmiTempCurrent,
            )
        );
        println!(
            "voltage data: {:?}",
            res.get_voltage_metric(RsmiVoltageMetric::RsmiVoltCurrent)
        );
        println!("busy percent: {:?}", res.get_busy_percent());
        println!("perf counters: {:?}", res.get_performance_countes());
        println!("perf level: {:?}", res.get_performance_level());
        println!("overdrive level: {:?}", res.get_overdrive_levels());
        println!(
            "freq core {:?}",
            res.get_frequency(RsmiClkType::RsmiClkTypeDf)
        );
        println!("f-v curve: {:?}", res.get_frequency_voltage_curve());
        println!("metrics: {:?}", res.get_full_metrics());

        Ok(())
    }
}

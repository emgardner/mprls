use super::core::{Address, Error, PressureUnit, MAXIMUM_PSI, MINIMUM_PSI, OUTPUT_MAX, OUTPUT_MIN};
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;

#[allow(dead_code)]
pub struct MPR<I2C, D> {
    i2c: I2C,
    address: Address,
    delay: D,
}

impl<I2C, D, E> MPR<I2C, D>
where
    I2C: I2c<Error = E>,
    D: DelayNs,
{
    pub fn new(i2c: I2C, address: Address, delay: D) -> Self {
        MPR {
            i2c,
            address,
            delay,
        }
    }

    pub fn check_status(&self, status_byte: u8) -> Result<(), Error<E>> {
        if status_byte & 0x20 > 1 {
            return Err(Error::Busy);
        }
        if status_byte & 0x04 > 1 {
            return Err(Error::IntegrityFailure);
        }
        if status_byte & 0x01 > 1 {
            return Err(Error::MathSaturation);
        }
        Ok(())
    }

    pub async fn start_measurement(&mut self) -> Result<(), Error<E>> {
        let cmd = [0xaa, 0x00, 0x00];
        self.i2c
            .write(self.address as u8, &cmd)
            .await
            .map_err(Error::I2cError)
    }

    pub async fn read_pressure(&mut self, pu: PressureUnit) -> Result<f32, Error<E>> {
        let mut buffer = [0, 0, 0, 0];
        let _ = self.i2c.read(self.address as u8, &mut buffer).await;
        self.check_status(buffer[0])?;
        let reading = ((buffer[1] as u32) << 16) + ((buffer[2] as u32) << 8) + buffer[3] as u32;

        let mut pressure = (reading as f32 - OUTPUT_MIN) * (MAXIMUM_PSI - MINIMUM_PSI);
        pressure = (pressure / (OUTPUT_MAX - OUTPUT_MIN)) + MINIMUM_PSI;
        match pu {
            PressureUnit::PSI => Ok(pressure),
            PressureUnit::PA => Ok(pressure * 6894.7573),
            PressureUnit::KPA => Ok(pressure * 6.89476),
            PressureUnit::TORR => Ok(pressure * 51.7149),
            PressureUnit::INHG => Ok(pressure * 2.03602),
            PressureUnit::ATM => Ok(pressure * 0.06805),
            PressureUnit::BAR => Ok(pressure * 0.06895),
        }
    }

    pub async fn get_pressure(&mut self, pu: PressureUnit) -> Result<f32, Error<E>> {
        self.start_measurement().await?;
        self.delay.delay_ms(20).await;
        self.read_pressure(pu).await
    }
}

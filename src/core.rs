/// Errors
#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error
    I2cError(E),
    Busy,
    IntegrityFailure,
    MathSaturation,
}

/// I²C address
#[derive(Copy, Clone)]
pub enum Address {
    Standard = 0x18,
    Alt1 = 0x08,
    Alt2 = 0x28,
    Alt3 = 0x38,
    Alt4 = 0x48,
    Alt5 = 0x58,
    Alt6 = 0x68,
    Alt7 = 0x78,
}

/// Pressure Units
pub enum PressureUnit {
    // PSI
    PSI,
    // Pa (Pascal)
    PA,
    // kPa (kilopascal)
    KPA,
    // torr (mmHg)
    TORR,
    // inHg (inch of mercury)
    INHG,
    // atm (atmosphere)
    ATM,
    // bar
    BAR,
}

pub const MAXIMUM_PSI: f32 = 25.0;
pub const MINIMUM_PSI: f32 = 0.0;

pub const OUTPUT_MAX: f32 = 0xE66666 as f32;
pub const OUTPUT_MIN: f32 = 0x19999A as f32;

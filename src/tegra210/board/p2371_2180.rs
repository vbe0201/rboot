use crate::tegra210::gpio::*;
use crate::tegra210::pinmux::*;

pub const GPIO_CONFIG: [(Gpio, GpioConfig); 59] = [
    (Gpio(GpioPort::A, 5), GpioConfig::Input),
    (Gpio(GpioPort::B, 0), GpioConfig::Input),
    (Gpio(GpioPort::B, 1), GpioConfig::Input),
    (Gpio(GpioPort::B, 2), GpioConfig::Input),
    (Gpio(GpioPort::B, 3), GpioConfig::Input),
    (Gpio(GpioPort::C, 0), GpioConfig::Input),
    (Gpio(GpioPort::C, 1), GpioConfig::Input),
    (Gpio(GpioPort::C, 2), GpioConfig::Input),
    (Gpio(GpioPort::C, 3), GpioConfig::Input),
    (Gpio(GpioPort::C, 4), GpioConfig::Input),
    (Gpio(GpioPort::E, 4), GpioConfig::Input),
    (Gpio(GpioPort::E, 5), GpioConfig::Input),
    (Gpio(GpioPort::E, 6), GpioConfig::Input),
    (Gpio(GpioPort::H, 0), GpioConfig::OutputLow),
    (Gpio(GpioPort::H, 1), GpioConfig::OutputLow),
    (Gpio(GpioPort::H, 2), GpioConfig::Input),
    (Gpio(GpioPort::H, 3), GpioConfig::OutputLow),
    (Gpio(GpioPort::H, 4), GpioConfig::OutputLow),
    (Gpio(GpioPort::H, 5), GpioConfig::Input),
    (Gpio(GpioPort::H, 6), GpioConfig::Input),
    (Gpio(GpioPort::H, 7), GpioConfig::Input),
    (Gpio(GpioPort::I, 0), GpioConfig::OutputLow),
    (Gpio(GpioPort::I, 1), GpioConfig::Input),
    (Gpio(GpioPort::I, 2), GpioConfig::OutputLow),
    (Gpio(GpioPort::K, 4), GpioConfig::Input),
    (Gpio(GpioPort::K, 5), GpioConfig::OutputLow),
    (Gpio(GpioPort::K, 6), GpioConfig::Input),
    (Gpio(GpioPort::K, 7), GpioConfig::Input),
    (Gpio(GpioPort::L, 1), GpioConfig::Input),
    (Gpio(GpioPort::S, 4), GpioConfig::OutputLow),
    (Gpio(GpioPort::S, 5), GpioConfig::OutputLow),
    (Gpio(GpioPort::S, 6), GpioConfig::OutputLow),
    (Gpio(GpioPort::S, 7), GpioConfig::OutputLow),
    (Gpio(GpioPort::T, 0), GpioConfig::OutputLow),
    (Gpio(GpioPort::T, 1), GpioConfig::OutputLow),
    (Gpio(GpioPort::U, 2), GpioConfig::Input),
    (Gpio(GpioPort::U, 3), GpioConfig::Input),
    (Gpio(GpioPort::V, 1), GpioConfig::OutputLow),
    (Gpio(GpioPort::V, 2), GpioConfig::OutputLow),
    (Gpio(GpioPort::V, 3), GpioConfig::Input),
    (Gpio(GpioPort::V, 5), GpioConfig::OutputLow),
    (Gpio(GpioPort::V, 6), GpioConfig::OutputLow),
    (Gpio(GpioPort::X, 0), GpioConfig::Input),
    (Gpio(GpioPort::X, 1), GpioConfig::Input),
    (Gpio(GpioPort::X, 2), GpioConfig::Input),
    (Gpio(GpioPort::X, 3), GpioConfig::Input),
    (Gpio(GpioPort::X, 4), GpioConfig::Input),
    (Gpio(GpioPort::X, 5), GpioConfig::Input),
    (Gpio(GpioPort::X, 6), GpioConfig::Input),
    (Gpio(GpioPort::X, 7), GpioConfig::Input),
    (Gpio(GpioPort::Y, 0), GpioConfig::Input),
    (Gpio(GpioPort::Y, 1), GpioConfig::Input),
    (Gpio(GpioPort::Z, 0), GpioConfig::Input),
    (Gpio(GpioPort::Z, 2), GpioConfig::Input),
    (Gpio(GpioPort::Z, 3), GpioConfig::OutputLow),
    (Gpio(GpioPort::BB, 0), GpioConfig::Input),
    (Gpio(GpioPort::BB, 2), GpioConfig::OutputLow),
    (Gpio(GpioPort::BB, 3), GpioConfig::Input),
    (Gpio(GpioPort::CC, 1), GpioConfig::Input),
];

pub const PINGRP_CONFIG: [(
    PinGrP,
    PinFunction,
    PinPull,
    PinTristate,
    PinIo,
    PinLock,
    PinOd,
    PinEIoHv,
); 161] = [
    (
        PinGrP::PexL0RstNPa0,
        PinFunction::Pe0,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::High,
    ),
    (
        PinGrP::PexL0ClkreqNPa1,
        PinFunction::Pe0,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::High,
    ),
    (
        PinGrP::PexWakeNPa2,
        PinFunction::Pe,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::High,
    ),
    (
        PinGrP::PexL1RstNPa3,
        PinFunction::Pe1,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::High,
    ),
    (
        PinGrP::PexL1ClkreqNPa4,
        PinFunction::Pe1,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::High,
    ),
    (
        PinGrP::SataLedActivePa5,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pa6,
        PinFunction::Sata,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Dap1FsPb0,
        PinFunction::Default,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Dap1DinPb1,
        PinFunction::Default,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Dap1DoutPb2,
        PinFunction::Default,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Dap1SclkPb3,
        PinFunction::Default,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Spi2MosiPb4,
        PinFunction::Spi2,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Spi2MisoPb5,
        PinFunction::Spi2,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Spi2SckPb6,
        PinFunction::Spi2,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Spi2Cs0Pb7,
        PinFunction::Spi2,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Spi1MosiPc0,
        PinFunction::Default,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Spi1MisoPc1,
        PinFunction::Default,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Spi1SckPc2,
        PinFunction::Default,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Spi1Cs0Pc3,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Spi1Cs1Pc4,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Spi4SckPc5,
        PinFunction::Spi4,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Spi4Cs0Pc6,
        PinFunction::Spi4,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Spi4MosiPc7,
        PinFunction::Spi4,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Spi4MisoPd0,
        PinFunction::Spi4,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Uart3TxPd1,
        PinFunction::Uartc,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Uart3RxPd2,
        PinFunction::Uartc,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Uart3RtsPd3,
        PinFunction::Uartc,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Uart3CtsPd4,
        PinFunction::Uartc,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Dmic1ClkPe0,
        PinFunction::I2S3,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Dmic1DatPe1,
        PinFunction::I2S3,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Dmic2ClkPe2,
        PinFunction::I2S3,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Dmic2DatPe3,
        PinFunction::I2S3,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Dmic3ClkPe4,
        PinFunction::Default,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Dmic3DatPe5,
        PinFunction::Default,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pe6,
        PinFunction::Default,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pe7,
        PinFunction::Pwm3,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Gen3I2CSclPf0,
        PinFunction::I2C3,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Normal,
    ),
    (
        PinGrP::Gen3I2CSdaPf1,
        PinFunction::I2C3,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Normal,
    ),
    (
        PinGrP::Uart2TxPg0,
        PinFunction::Uartb,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Uart2RxPg1,
        PinFunction::Uartb,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Uart2RtsPg2,
        PinFunction::Uartb,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Uart2CtsPg3,
        PinFunction::Uartb,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::WifiEnPh0,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::WifiRstPh1,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::WifiWakeApPh2,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::ApWakeBtPh3,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::BtRstPh4,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::BtWakeApPh5,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Ph6,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::ApWakeNfcPh7,
        PinFunction::Default,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::NfcEnPi0,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::NfcIntPi1,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::GpsEnPi2,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::GpsRstPi3,
        PinFunction::Rsvd0,
        PinPull::Down,
        PinTristate::Tristate,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Uart4TxPi4,
        PinFunction::Uartd,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Uart4RxPi5,
        PinFunction::Uartd,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Uart4RtsPi6,
        PinFunction::Uartd,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Uart4CtsPi7,
        PinFunction::Uartd,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Gen1I2CSdaPj0,
        PinFunction::I2C1,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Normal,
    ),
    (
        PinGrP::Gen1I2CSclPj1,
        PinFunction::I2C1,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Normal,
    ),
    (
        PinGrP::Gen2I2CSclPj2,
        PinFunction::I2C2,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::High,
    ),
    (
        PinGrP::Gen2I2CSdaPj3,
        PinFunction::I2C2,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::High,
    ),
    (
        PinGrP::Dap4FsPj4,
        PinFunction::I2S4B,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Dap4DinPj5,
        PinFunction::I2S4B,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Dap4DoutPj6,
        PinFunction::I2S4B,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Dap4SclkPj7,
        PinFunction::I2S4B,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pk0,
        PinFunction::I2S5B,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pk1,
        PinFunction::I2S5B,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pk2,
        PinFunction::I2S5B,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pk3,
        PinFunction::I2S5B,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pk4,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pk5,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pk6,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pk7,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pl0,
        PinFunction::Rsvd0,
        PinPull::Down,
        PinTristate::Tristate,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pl1,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Sdmmc1ClkPm0,
        PinFunction::Sdmmc1,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Sdmmc1CmdPm1,
        PinFunction::Sdmmc1,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Sdmmc1Dat3Pm2,
        PinFunction::Sdmmc1,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Sdmmc1Dat2Pm3,
        PinFunction::Sdmmc1,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Sdmmc1Dat1Pm4,
        PinFunction::Sdmmc1,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Sdmmc1Dat0Pm5,
        PinFunction::Sdmmc1,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Sdmmc3ClkPp0,
        PinFunction::Sdmmc3,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Sdmmc3CmdPp1,
        PinFunction::Sdmmc3,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Sdmmc3Dat3Pp2,
        PinFunction::Sdmmc3,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Sdmmc3Dat2Pp3,
        PinFunction::Sdmmc3,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Sdmmc3Dat1Pp4,
        PinFunction::Sdmmc3,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Sdmmc3Dat0Pp5,
        PinFunction::Sdmmc3,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Cam1MclkPs0,
        PinFunction::Extperiph3,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Cam2MclkPs1,
        PinFunction::Extperiph3,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::CamI2CSclPs2,
        PinFunction::I2Cvi,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Normal,
    ),
    (
        PinGrP::CamI2CSdaPs3,
        PinFunction::I2Cvi,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Normal,
    ),
    (
        PinGrP::CamRstPs4,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::CamAfEnPs5,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::CamFlashEnPs6,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Cam1PwdnPs7,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Cam2PwdnPt0,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Cam1StrobePt1,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Uart1TxPu0,
        PinFunction::Uarta,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Uart1RxPu1,
        PinFunction::Uarta,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Uart1RtsPu2,
        PinFunction::Default,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Uart1CtsPu3,
        PinFunction::Default,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::LcdBlPwmPv0,
        PinFunction::Pwm0,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::LcdBlEnPv1,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::LcdRstPv2,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::LcdGpio1Pv3,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::LcdGpio2Pv4,
        PinFunction::Pwm1,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::ApReadyPv5,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::TouchRstPv6,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::TouchClkPv7,
        PinFunction::Touch,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::ModemWakeApPx0,
        PinFunction::Default,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::TouchIntPx1,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::MotionIntPx2,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::AlsProxIntPx3,
        PinFunction::Default,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::TempAlertPx4,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::ButtonPowerOnPx5,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::ButtonVolUpPx6,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::ButtonVolDownPx7,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::ButtonSlideSwPy0,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::ButtonHomePy1,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::LcdTePy2,
        PinFunction::Displaya,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::PwrI2CSclPy3,
        PinFunction::I2Cpmu,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Normal,
    ),
    (
        PinGrP::PwrI2CSdaPy4,
        PinFunction::I2Cpmu,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Normal,
    ),
    (
        PinGrP::Clk32KOutPy5,
        PinFunction::Soc,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pz0,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pz1,
        PinFunction::Sdmmc1,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pz2,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pz3,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pz4,
        PinFunction::Sdmmc1,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pz5,
        PinFunction::Soc,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Dap2FsPaa0,
        PinFunction::I2S2,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Dap2SclkPaa1,
        PinFunction::I2S2,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Dap2DinPaa2,
        PinFunction::I2S2,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Dap2DoutPaa3,
        PinFunction::I2S2,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::AudMclkPbb0,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::DvfsPwmPbb1,
        PinFunction::Cldvfs,
        PinPull::Normal,
        PinTristate::Tristate,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::DvfsClkPbb2,
        PinFunction::Default,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::GpioX1AudPbb3,
        PinFunction::Default,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::GpioX3AudPbb4,
        PinFunction::Rsvd0,
        PinPull::Down,
        PinTristate::Tristate,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::HdmiCecPcc0,
        PinFunction::Cec,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::High,
    ),
    (
        PinGrP::HdmiIntDpHpdPcc1,
        PinFunction::Default,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Normal,
    ),
    (
        PinGrP::SpdifOutPcc2,
        PinFunction::Rsvd1,
        PinPull::Down,
        PinTristate::Tristate,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::SpdifInPcc3,
        PinFunction::Rsvd1,
        PinPull::Down,
        PinTristate::Tristate,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::UsbVbusEn0Pcc4,
        PinFunction::Usb,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::High,
    ),
    (
        PinGrP::UsbVbusEn1Pcc5,
        PinFunction::Usb,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::High,
    ),
    (
        PinGrP::DpHpd0Pcc6,
        PinFunction::Dp,
        PinPull::Down,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Pcc7,
        PinFunction::Rsvd0,
        PinPull::Down,
        PinTristate::Tristate,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Normal,
    ),
    (
        PinGrP::Spi2Cs1Pdd0,
        PinFunction::Spi2,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::QspiSckPee0,
        PinFunction::Rsvd1,
        PinPull::Down,
        PinTristate::Tristate,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::QspiCsNPee1,
        PinFunction::Rsvd1,
        PinPull::Down,
        PinTristate::Tristate,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::QspiIo0Pee2,
        PinFunction::Rsvd1,
        PinPull::Down,
        PinTristate::Tristate,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::QspiIo1Pee3,
        PinFunction::Rsvd1,
        PinPull::Down,
        PinTristate::Tristate,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::QspiIo2Pee4,
        PinFunction::Rsvd1,
        PinPull::Down,
        PinTristate::Tristate,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::QspiIo3Pee5,
        PinFunction::Rsvd1,
        PinPull::Down,
        PinTristate::Tristate,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::CorePwrReq,
        PinFunction::Core,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::CpuPwrReq,
        PinFunction::Cpu,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::PwrIntN,
        PinFunction::Pmi,
        PinPull::Up,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Clk32KIn,
        PinFunction::Clk,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Input,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::JtagRtck,
        PinFunction::Jtag,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::ClkReq,
        PinFunction::Rsvd1,
        PinPull::Down,
        PinTristate::Tristate,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
    (
        PinGrP::Shutdown,
        PinFunction::Shutdown,
        PinPull::Normal,
        PinTristate::Normal,
        PinIo::Output,
        PinLock::Default,
        PinOd::Disable,
        PinEIoHv::Default,
    ),
];

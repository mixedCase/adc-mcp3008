//! A platform agnostic driver to interface with the MCP3008 / MCP3004 ADC's.
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! When the "async" feature is enabled, async support is provided via [`embedded-hal-async`].
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/1.0.0
//! [`embedded-hal-async`]: https://docs.rs/embedded-hal-async/1.0.0
//!

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate embedded_hal;
#[cfg(feature = "async")]
extern crate embedded_hal_async;

use embedded_hal::spi::{SpiDevice, Mode, Phase, Polarity};
#[cfg(feature = "async")]
use embedded_hal_async::spi::SpiDevice as AsyncSpiDevice;

/// SPI mode
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

/// Synchronous MCP3008 driver
pub struct Mcp3008<SpiDev> {
    spi_dev: SpiDev,
}

#[cfg(feature = "async")]
/// Asynchronous MCP3008 driver
///
/// This struct is only available when the "async" feature is enabled.
pub struct AsyncMcp3008<SpiDev> {
    spi_dev: SpiDev,
}

/// Synchronous MCP3004 driver
pub struct Mcp3004<SpiDev> {
    spi_dev: SpiDev,
}

#[cfg(feature = "async")]
/// Asynchronous MCP3004 driver
///
/// This struct is only available when the "async" feature is enabled.
pub struct AsyncMcp3004<SpiDev> {
    spi_dev: SpiDev,
}

impl<SpiDev> Mcp3008<SpiDev>
where
    SpiDev: SpiDevice,
{
    /// Creates a new driver from an SPI device.
    pub fn new(spi_dev: SpiDev) -> Self {
        Mcp3008 { spi_dev }
    }

    /// Read a MCP3008 ADC channel and return the 10 bit value as a u16
    pub fn read_channel(&mut self, ch: Channels8) -> Result<u16, SpiDev::Error> {
        let write_buffer = [1, ((1 << 3) | (ch as u8)) << 4, 0];
        let mut read_buffer = [0u8; 3];

        self.spi_dev.transaction(&mut [
            embedded_hal::spi::Operation::Write(&write_buffer),
            embedded_hal::spi::Operation::Read(&mut read_buffer),
        ])?;

        let r = (((read_buffer[1] as u16) << 8) | (read_buffer[2] as u16)) & 0x3ff;
        Ok(r)
    }
}

#[cfg(feature = "async")]
impl<SpiDev> AsyncMcp3008<SpiDev>
where
    SpiDev: AsyncSpiDevice,
{
    /// Creates a new async driver from an SPI device.
    pub fn new(spi_dev: SpiDev) -> Self {
        AsyncMcp3008 { spi_dev }
    }

    /// Read a MCP3008 ADC channel and return the 10 bit value as a u16
    pub async fn read_channel(&mut self, ch: Channels8) -> Result<u16, SpiDev::Error> {
        let write_buffer = [1, ((1 << 3) | (ch as u8)) << 4, 0];
        let mut read_buffer = [0u8; 3];

        self.spi_dev.transaction(&mut [
            embedded_hal_async::spi::Operation::Write(&write_buffer),
            embedded_hal_async::spi::Operation::Read(&mut read_buffer),
        ]).await?;

        let r = (((read_buffer[1] as u16) << 8) | (read_buffer[2] as u16)) & 0x3ff;
        Ok(r)
    }
}

impl<SpiDev> Mcp3004<SpiDev>
where
    SpiDev: SpiDevice,
{
    /// Creates a new driver from an SPI device.
    pub fn new(spi_dev: SpiDev) -> Self {
        Mcp3004 { spi_dev }
    }

    /// Read a MCP3004 ADC channel and return the 10 bit value as a u16
    pub fn read_channel(&mut self, ch: Channels4) -> Result<u16, SpiDev::Error> {
        let write_buffer = [1, ((1 << 3) | (ch as u8)) << 4, 0];
        let mut read_buffer = [0u8; 3];

        self.spi_dev.transaction(&mut [
            embedded_hal::spi::Operation::Write(&write_buffer),
            embedded_hal::spi::Operation::Read(&mut read_buffer),
        ])?;

        let r = (((read_buffer[1] as u16) << 8) | (read_buffer[2] as u16)) & 0x3ff;
        Ok(r)
    }
}

#[cfg(feature = "async")]
impl<SpiDev> AsyncMcp3004<SpiDev>
where
    SpiDev: AsyncSpiDevice,
{
    /// Creates a new async driver from an SPI device.
    pub fn new(spi_dev: SpiDev) -> Self {
        AsyncMcp3004 { spi_dev }
    }

    /// Read a MCP3004 ADC channel and return the 10 bit value as a u16
    pub async fn read_channel(&mut self, ch: Channels4) -> Result<u16, SpiDev::Error> {
        let write_buffer = [1, ((1 << 3) | (ch as u8)) << 4, 0];
        let mut read_buffer = [0u8; 3];

        self.spi_dev.transaction(&mut [
            embedded_hal_async::spi::Operation::Write(&write_buffer),
            embedded_hal_async::spi::Operation::Read(&mut read_buffer),
        ]).await?;

        let r = (((read_buffer[1] as u16) << 8) | (read_buffer[2] as u16)) & 0x3ff;
        Ok(r)
    }
}

/// Channel list for MCP3008
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub enum Channels8 {
    CH0,
    CH1,
    CH2,
    CH3,
    CH4,
    CH5,
    CH6,
    CH7,
}

/// Channel list for MCP3004
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub enum Channels4 {
    CH0,
    CH1,
    CH2,
    CH3,
}

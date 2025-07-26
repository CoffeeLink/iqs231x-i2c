use crate::Iqs231xError;
use embedded_hal::i2c::SevenBitAddress;

/// The default address of the IQS231A/B chips on I2C
pub const DEFAULT_ADDR: SevenBitAddress = 0x44;


// register addrs:
const PRODUCT_NUMBER_REG: u8 = 0x00;

#[derive(Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct Iqs231xDriver<I2C> {
    address: SevenBitAddress,
    i2c: I2C
}

#[warn(missing_docs)]
impl <I2C> Iqs231xDriver<I2C> {
    /// Creates a new IQS231X driver instance with the default I2C address (0x44).
    ///
    /// If a custom address is needed, use the [`with_address`](Iqs231xDriver::with_address) function instead.
    /// Note: The I2C address can be changed later using the [`set_address`](Iqs231xDriver::set_address) function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use iqs231x_i2c::Iqs231xDriver;
    /// # let i2c_interface = embedded_hal_mock::eh1::i2c::Mock::new(&[]);
    ///
    /// let sensor = Iqs231xDriver::new(i2c_interface);
    /// # sensor.release_inner().done();
    /// ```
    pub fn new(i2c: I2C) -> Self {
        Self {
            address: DEFAULT_ADDR,
            i2c
        }
    }

    /// Creates a new driver instance with a custom I2C address.
    ///
    /// # Arguments
    ///
    /// - `i2c` - The I2C interface
    /// - `addr` - Custom 7-bit I2C address
    ///
    /// # Example
    ///
    /// ```rust
    /// use iqs231x_i2c::Iqs231xDriver;
    /// # let i2c_interface = embedded_hal_mock::eh1::i2c::Mock::new(&[]);
    ///
    /// let custom_address = 0x45;
    /// let sensor = Iqs231xDriver::with_address(i2c_interface, custom_address);
    ///
    /// assert_eq!(sensor.address(), 0x45u8);
    /// # sensor.release_inner().done();
    /// ```
    pub fn with_address(i2c: I2C, addr: SevenBitAddress) -> Self {
        Self {
            address: addr,
            i2c,
        }
    }

    /// Updates the device's I2C address.
    ///
    /// # Arguments
    ///
    /// - `addr` - New 7-bit I2C address
    ///
    /// # Example
    ///
    /// ```rust
    /// use embedded_hal::i2c::SevenBitAddress;
    /// use iqs231x_i2c::Iqs231xDriver;
    /// # let i2c_interface = embedded_hal_mock::eh1::i2c::Mock::new(&[]);
    ///
    /// let mut sensor = Iqs231xDriver::new(i2c_interface);
    /// sensor.set_address(SevenBitAddress::from(0x45));
    ///
    /// assert_eq!(sensor.address(), SevenBitAddress::from(0x45));
    /// # sensor.release_inner().done();
    /// ```
    pub fn set_address(&mut self, addr: SevenBitAddress) {
        self.address = addr;
    }

    /// Returns the current I2C address of the device.
    ///
    /// # Returns
    ///
    /// The current 7-bit I2C address
    ///
    /// # Example
    ///
    /// ```rust
    /// # use embedded_hal::i2c::SevenBitAddress;
    /// use iqs231x_i2c::Iqs231xDriver;
    ///
    /// # let i2c_interface = embedded_hal_mock::eh1::i2c::Mock::new(&[]);
    /// let sensor = Iqs231xDriver::with_address(i2c_interface, SevenBitAddress::from(0x10));
    /// assert_eq!(sensor.address(), SevenBitAddress::from(0x10));
    /// # sensor.release_inner().done();
    /// ```
    pub fn address(&self) -> SevenBitAddress {
        self.address
    }

    /// Consumes the driver and returns the underlying I2C peripheral.
    ///
    /// This is useful when you need to reuse the I2C interface for other purposes
    /// after you're done with the sensor.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iqs231x_i2c::Iqs231xDriver;
    /// # let i2c_interface = embedded_hal_mock::eh1::i2c::Mock::new(&[]);
    ///
    /// let sensor = Iqs231xDriver::new(i2c_interface);
    /// // ... use the sensor
    /// let i2c = sensor.release_inner(); // Get back the I2C interface
    /// # let mut i2c = i2c; // .done() requires mut
    /// # i2c.done()
    /// ```
    pub fn release_inner(self) -> I2C {
        self.i2c
    }
}

#[cfg(feature = "blocking")]
impl<I2C, E> Iqs231xDriver<I2C>
where I2C: embedded_hal::i2c::I2c<Error = E>
{
    pub fn product_number(&mut self) -> Result<u8, Iqs231xError<E>> {
        let mut results: [u8; 2] = [0, 0];

        self.i2c.write_read(self.address, &[PRODUCT_NUMBER_REG], &mut results)?;

        Ok(results[1])
    }
}

#[cfg(feature = "async")]
impl<I2C, E> Iqs231xDriver<I2C>
where I2C: embedded_hal_async::i2c::I2c<Error = E>
{
    pub async fn product_number(&mut self) -> Result<u8, Iqs231xError<E>> {
        let mut results: [u8; 2] = [0, 0];

        self.i2c.write_read(self.address, &[PRODUCT_NUMBER_REG], &mut results).await?;

        Ok(results[1])
    }
}

#[cfg(test)]
mod tests {
    use crate::iqs231x::DEFAULT_ADDR;
    use crate::Iqs231xDriver;
    use alloc::vec;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    #[test]
    fn test_product_number() {
        let mut expectations = vec![];

        expectations.extend_from_slice(&[
            Transaction::write_read(DEFAULT_ADDR, vec![0x00], vec![0x00, 0x40])
        ]);

        let mock = Mock::new(&expectations);

        let mut sensor = Iqs231xDriver::new(mock);
        let num = sensor.product_number().expect("Errored");

        assert_eq!(num, 0x40);

        sensor.release_inner().done();
    }

}
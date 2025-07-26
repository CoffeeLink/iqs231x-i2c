
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Iqs231xError<E> {
    I2CError(E)
}

impl<E> From<E> for Iqs231xError<E> {
    fn from(value: E) -> Self {
        Iqs231xError::I2CError(value)
    }
}
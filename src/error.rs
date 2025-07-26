
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Iqs231xError<E> {

    Other(E)
}

impl<E> From<E> for Iqs231xError<E> {
    fn from(value: E) -> Self {
        Iqs231xError::Other(value)
    }
}
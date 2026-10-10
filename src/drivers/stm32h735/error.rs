#[derive(Debug)]
pub enum Error<T> {
    I2c(T),
    PoorData,
}

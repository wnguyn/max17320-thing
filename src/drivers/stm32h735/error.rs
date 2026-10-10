// error enum for Result<T, E>
#[derive(Debug)]
pub enum Error<T> {
    I2c(T),
    PoorData,
}

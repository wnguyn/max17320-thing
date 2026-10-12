#[derive(Debug)]
pub enum BmsError<T> {
    I2c(T),
    SixSevenData,
}

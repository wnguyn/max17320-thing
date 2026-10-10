// generic type for more portability
#[derive(Debug)]
pub enum BmsError<T> {
    I2c(T),
    SixSevenData,
}

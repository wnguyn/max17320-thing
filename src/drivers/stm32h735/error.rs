// i don't exactly know how this is gonna be used, it just seems nice to have an enum for this because I2C can be weird iirc..
pub enum BmsError<T> {
    I2c(T),
    SixSevenData,
}

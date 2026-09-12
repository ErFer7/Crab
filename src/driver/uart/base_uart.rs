use crate::driver::driver::Driver;

pub trait BaseUART {
    fn write(data: char);

    fn read() -> char;
}

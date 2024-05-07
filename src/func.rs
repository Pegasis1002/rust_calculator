pub mod math_funcs {
    pub fn add(x: i64, y: i64) -> i64 {
        x + y
    }

    pub fn sub(x: i64, y: i64) -> i64 {
        x - y
    }

    pub fn multiply(x: i64, y: i64) -> i64 {
        x * y
    }

    pub fn divide(x: i64, y: i64) -> i64 {
        x / y
    }

    pub fn remender(x: i64, y: i64) -> i64 {
        x % y
    }

    pub fn pow(x: i64, y: i64) -> i64 {
        x.pow(y as u32)
    }
}

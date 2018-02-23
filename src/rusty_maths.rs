#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(4.0, add(2.0, 2.0));
    }

    #[test]
    fn test_multiply() {
        assert_eq!(8.0, multiply(4.0, 2.0));
    }

    #[test]
    fn test_subtract() {
        assert_eq!(8.0, subtract(10.0, 2.0));
    }

    #[test]
    fn test_divide() {
        assert_eq!(50.0, divide(100.0, 2.0));
    }
}

pub fn add(x: f64, y: f64) -> f64 {
    return x + y;
}

pub fn multiply(x: f64, y: f64) -> f64 {
    return x * y;
}

pub fn subtract(x: f64, y: f64) -> f64 {
    return x - y;
}

pub fn divide(x: f64, y:f64) -> f64 {
    return x / y;
}
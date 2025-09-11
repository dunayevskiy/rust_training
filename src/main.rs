pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    println!("🦀 Rust Calculator Starting!");
    println!("Add: 10 + 5 = {}", add(10, 5));
    println!("Subtract: 10 - 3 = {}", subtract(10, 3));
    println!("Multiply: 4 * 7 = {}", multiply(4, 7));
    
    match divide(20, 4) {
        Ok(result) => println!("Divide: 20 / 4 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("Calculator finished!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
        assert_eq!(subtract(1, 1), 0);
        assert_eq!(subtract(0, 5), -5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 5), 20);
        assert_eq!(multiply(-2, 3), -6);
        assert_eq!(multiply(0, 100), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(7, 3), Ok(2)); // Integer division
        assert_eq!(divide(5, 0), Err("Division by zero".to_string()));
    }
}
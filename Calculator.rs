use std::io;

fn main() {
    println!("Simple Calculator");
    println!("Available operations: +, -, *, /, %, ^ (power)");
    println!("Type 'quit' to exit\n");

    loop {
        println!("Enter first number:");
        let num1 = match get_number() {
            Some(n) => n,
            None => continue,
        };

        println!("Enter operator (+, -, *, /, %, ^):");
        let operator = get_operator();
        if operator.is_empty() {
            continue;
        }

        println!("Enter second number:");
        let num2 = match get_number() {
            Some(n) => n,
            None => continue,
        };

        let result = calculate(num1, &operator, num2);
        match result {
            Ok(value) => println!("Result: {}\n", value),
            Err(error) => println!("Error: {}\n", error),
        }
    }
}

fn get_number() -> Option<f64> {
    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim();
            
            if trimmed.to_lowercase() == "quit" {
                println!("Goodbye!");
                std::process::exit(0);
            }
            
            match trimmed.parse::<f64>() {
                Ok(num) => Some(num),
                Err(_) => {
                    println!("Invalid number. Please try again.");
                    None
                }
            }
        }
        Err(_) => {
            println!("Failed to read input. Please try again.");
            None
        }
    }
}

fn get_operator() -> String {
    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim();
            
            if trimmed.to_lowercase() == "quit" {
                println!("Goodbye!");
                std::process::exit(0);
            }
            
            match trimmed {
                "+" | "-" | "*" | "/" | "%" | "^" => trimmed.to_string(),
                _ => {
                    println!("Invalid operator. Please use +, -, *, /, %, or ^");
                    String::new()
                }
            }
        }
        Err(_) => {
            println!("Failed to read input. Please try again.");
            String::new()
        }
    }
}

fn calculate(num1: f64, operator: &str, num2: f64) -> Result<f64, String> {
    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Cannot divide by zero".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        "%" => {
            if num2 == 0.0 {
                Err("Cannot calculate modulo with zero".to_string())
            } else {
                Ok(num1 % num2)
            }
        }
        "^" => Ok(num1.powf(num2)),
        _ => Err("Invalid operator".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(calculate(2.0, "+", 3.0).unwrap(), 5.0);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calculate(5.0, "-", 3.0).unwrap(), 2.0);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculate(4.0, "*", 3.0).unwrap(), 12.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(calculate(10.0, "/", 2.0).unwrap(), 5.0);
    }

    #[test]
    fn test_division_by_zero() {
        assert!(calculate(10.0, "/", 0.0).is_err());
    }

    #[test]
    fn test_modulo() {
        assert_eq!(calculate(10.0, "%", 3.0).unwrap(), 1.0);
    }

    #[test]
    fn test_power() {
        assert_eq!(calculate(2.0, "^", 3.0).unwrap(), 8.0);
    }
}

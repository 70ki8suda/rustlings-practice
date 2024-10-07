#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.
        if value < 0 {
            return Err(CreationError::Negative);
        } else if value == 0 {
            return Err(CreationError::Zero);
        }
        Ok(Self(value as u64))
    }
}

fn main() {
    // You can optionally experiment here.
    let test_values = vec![10, -5, 0, 25];
    
    for value in test_values {
        match PositiveNonzeroInteger::new(value) {
            Ok(positive) => println!("Successfully created: {:?}", positive),
            Err(e) => println!("Failed to create PositiveNonzeroInteger: {:?}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}

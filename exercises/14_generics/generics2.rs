// This powerful wrapper provides the ability to store a positive integer value.
// TODO: Rewrite it using a generic so that it supports wrapping ANY type.
struct Wrapper<T> {
    value: T,
}

// Adapt the struct's implementation to be generic over the wrapped value.
impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // You can optionally experiment here.
    // Experiment with storing different types in the Wrapper
    let int_wrapper = Wrapper::new(100);
    println!("Wrapped integer: {}", int_wrapper.value);

    let float_wrapper = Wrapper::new(std::f64::consts::PI);
    println!("Wrapped float: {}", float_wrapper.value);

    let string_wrapper = Wrapper::new(String::from("Hello, World!"));
    println!("Wrapped string: {}", string_wrapper.value);

    let bool_wrapper = Wrapper::new(true);
    println!("Wrapped boolean: {}", bool_wrapper.value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}

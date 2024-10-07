trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct;
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

struct OtherStruct;
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// TODO: Fix the compiler error by only changing the signature of this function.
fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    // You can optionally experiment here.
    let some_struct = SomeStruct;
    let result1 = some_func(some_struct);
    println!("Result with SomeStruct: {}", result1);

    let other_struct = OtherStruct;
    let result2 = some_func(other_struct);
    println!("Result with OtherStruct: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func() {
        assert!(some_func(SomeStruct));
        assert!(some_func(OtherStruct));
    }
}

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}

fn main() {
    // You can optionally experiment here.
    let mut my_vec = vec![String::from("Hello"), String::from("World")];
    println!("Before append_bar: {:?}", my_vec);

    my_vec = my_vec.append_bar();
    println!("After append_bar: {:?}", my_vec);

    let mut another_vec = vec![String::from("Rust")];
    println!("Before append_bar: {:?}", another_vec);

    another_vec = another_vec.append_bar();
    println!("After append_bar: {:?}", another_vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}

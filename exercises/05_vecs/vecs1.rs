fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    let v = vec![10,20,30,40];

    // この行は、関数 array_and_vec が配列 a とベクタ v をタプルとして返すことを示しています。
    (a, v)
}

fn main() {
    // You can optionally experiment here.
    array_and_vec();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}

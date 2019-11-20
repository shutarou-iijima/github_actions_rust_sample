fn main() {
    println!("{}", add(1, 2));
}

fn add(a: i32, b: i32) -> i32 {
   a + b
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add() {
        let value = add(1, 2);
        assert_eq!(3, value);
    }
}

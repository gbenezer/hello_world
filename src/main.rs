fn main() {
    println!("Hello, world!");
    println!("Two plus two equals {}", addition(2, 2))
}

fn addition(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_works() {
        let result = addition(2, 2);
        assert_eq!(result, 4);
    }
}
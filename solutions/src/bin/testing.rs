fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    add(2, 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit() {
        assert_eq!(add(1, 2), 3);
    }
}

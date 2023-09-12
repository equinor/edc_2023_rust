fn divide(a: i64, b: i64) -> Option<i64> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

fn main() {
    println!("{:?}", divide(5, 2));
}

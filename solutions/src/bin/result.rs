use std::num::ParseIntError;

fn to_number(s: &str) -> Result<i64, ParseIntError> {
    let a: i64 = s.parse()?;
    Ok(a + 1)
}

fn main() {
    println!("{:?}", to_number("41"))
}

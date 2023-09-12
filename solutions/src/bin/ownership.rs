fn main() {
    let s1 = String::from("hello");
    // let _s2 = s1.clone();
    let _s2 = &s1;

    println!("{}, world!", s1);
}

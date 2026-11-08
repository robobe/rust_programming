fn main() {
    let fixed: [i32; 3] = [2, 4, 6];
    let (count, total): (usize, i64) = (fixed.len(), 12);
    assert_eq!(fixed[0], 2);
    assert_eq!((count, total), (3, 12));
    let mut numbers = vec![3, -1, 8];
    numbers.push(2);
    assert_eq!(numbers.len(), 4);
    let greeting: &str = "Hello";
    let mut title = String::from(greeting);
    title.push_str(", Rust");
    assert_eq!(title, "Hello, Rust");
    assert_eq!("é".len(), 2);
    assert_eq!("é".chars().count(), 1);
    println!("count: {count}, total: {total}, values: {numbers:?}");
    println!("{title}");
}

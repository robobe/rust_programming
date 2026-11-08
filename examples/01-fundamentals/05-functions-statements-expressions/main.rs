fn double(value: i32) -> i32 {
    value * 2
}

fn nonnegative(value: i32) -> i32 {
    if value < 0 {
        return 0;
    }
    value
}

fn main() {
    assert_eq!(double(3), 6);
    assert_eq!(double(0), 0);
    assert_eq!(double(-2), -4);
    assert_eq!(nonnegative(-2), 0);
    assert_eq!(nonnegative(3), 3);
    println!("double: {}, clamped: {}", double(3), nonnegative(-2));
}

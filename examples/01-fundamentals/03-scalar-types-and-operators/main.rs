fn main() {
    let count: i32 = 5;
    let divisor: i32 = 2;
    let whole = count / divisor;
    let fraction = 5.0_f64 / 2.0;
    let enabled: bool = true;
    let initial: char = 'λ';
    assert_eq!(whole, 2);
    assert_eq!(count % divisor, 1);
    assert_eq!(fraction, 2.5);
    assert!(enabled && count > 0);
    assert_eq!(initial, 'λ');
    println!("whole: {whole}, fraction: {fraction}, initial: {initial}");
}

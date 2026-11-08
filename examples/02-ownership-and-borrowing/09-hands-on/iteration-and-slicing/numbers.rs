fn sum_values(values: &[i32]) -> i32 {
    let mut total = 0;
    for value in values.iter() {
        total += *value;
    }
    total
}

fn add_one_to_each(values: &mut [i32]) {
    for value in values.iter_mut() {
        *value += 1;
    }
}

fn main() {
    let mut numbers = vec![10, 20, 30, 40];
    let total = sum_values(&numbers[1..3]);
    assert_eq!(total, 50);
    add_one_to_each(&mut numbers[1..3]);
    assert_eq!(numbers, vec![10, 21, 31, 40]);
    println!("middle sum: {total}");
    println!("updated: {numbers:?}");

    let mut array = [1, 2];
    assert_eq!(sum_values(&array), 3);
    add_one_to_each(&mut array);
    assert_eq!(array, [2, 3]);

    let mut empty: [i32; 0] = [];
    assert_eq!(sum_values(&empty), 0);
    add_one_to_each(&mut empty);
    assert_eq!(empty, []);
}

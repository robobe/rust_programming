fn sum_values(values: &[i32]) -> i32 {
    // `iter` borrows each element, so the caller keeps the Vec.
    values.iter().sum()
}

fn add_one_to_each(values: &mut [i32]) {
    // `iter_mut` gives one mutable borrow of each element in turn.
    for value in values.iter_mut() {
        *value += 1;
    }
}

fn main() {
    let mut numbers = vec![1, 2, 3];

    let total = sum_values(&numbers);
    println!("total: {total}");

    add_one_to_each(&mut numbers);
    println!("updated: {numbers:?}");

    // `into_iter` consumes the Vec and yields owned elements.
    let owned_values: Vec<i32> = numbers.into_iter().map(|value| value * 2).collect();
    println!("consumed into new Vec: {owned_values:?}");
}

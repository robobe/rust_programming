fn main() {
    let mut total = 0;
    for number in 1..=3 {
        total += number;
    }
    assert_eq!(total, 6);
    let mut remaining = 2;
    while remaining > 0 {
        remaining -= 1;
    }
    assert_eq!(remaining, 0);
    let mut attempts = 0;
    let finished = loop {
        attempts += 1;
        if attempts == 2 {
            break attempts;
        }
    };
    assert_eq!(finished, 2);
    let mut odd_total = 0;
    for number in 1..=4 {
        if number % 2 == 0 {
            continue;
        }
        odd_total += number;
    }
    assert_eq!(odd_total, 4);
    println!("sum: {total}, remaining: {remaining}, finished: {finished}, odd sum: {odd_total}");
    let mut countdown = Vec::new();
    for number in (1..=4).rev() {
        countdown.push(number);
        println!("{number}");
    }
    assert_eq!(countdown, [4, 3, 2, 1]);
}

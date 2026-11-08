fn summarize(values: &[i32]) -> (usize, i64) {
    // A slice lets us read either an array or a vector without consuming it.
    let mut total: i64 = 0;
    for &value in values {
        // Widen each i32 before adding; keep the demo inputs small.
        total += i64::from(value);
    }
    (values.len(), total) // No semicolon: this tuple is the returned value.
}

fn size_label(count: usize) -> &'static str {
    match count {
        0 => "empty",
        1 => "single",
        _ => "multiple",
    }
}

fn main() {
    let fixed = [2, 4, 6];
    let mut numbers = vec![3, -1, 8];
    numbers.push(2); // Vec can grow; arrays have a fixed length.

    let mut title = String::from("Number");
    title.push_str(" summary");
    println!("{title}");

    let (count, total) = summarize(&numbers);
    println!("count: {count}, total: {total}");
    println!("size: {}", size_label(count));
    println!("positive total: {}", total > 0);

    let (array_count, array_total) = summarize(&fixed);
    println!("array count: {array_count}, total: {array_total}");

    let mut remaining = 2;
    while remaining > 0 {
        println!("countdown: {remaining}");
        remaining -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_mixed_values() {
        assert_eq!(summarize(&[3, -1, 8, 2]), (4, 12));
    }

    #[test]
    fn summarizes_empty_input() {
        assert_eq!(summarize(&[]), (0, 0));
    }

    #[test]
    fn summarizes_negative_values() {
        assert_eq!(summarize(&[-3, -2]), (2, -5));
    }

    #[test]
    fn widens_before_addition() {
        assert_eq!(summarize(&[i32::MAX, i32::MAX]), (2, 4_294_967_294));
    }

    #[test]
    fn classifies_sizes() {
        assert_eq!(size_label(0), "empty");
        assert_eq!(size_label(1), "single");
        assert_eq!(size_label(3), "multiple");
    }
}

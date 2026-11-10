fn identity<T>(value: T) -> T {
    value
}

fn first<T>(values: &[T]) -> Option<&T> {
    values.first()
}

fn main() {
    let number = identity(7_i32);
    let text = identity(String::from("Rust"));
    let explicit = identity::<u8>(9);
    assert_eq!(number, 7);
    assert_eq!(text, "Rust");
    assert_eq!(explicit, 9);
    println!("identity: {number}, {text}, {explicit}");

    let words = [String::from("Read"), String::from("Practice")];
    assert_eq!(first(&words), Some(&words[0]));
    assert_eq!(first::<String>(&[]), None);
    assert_eq!(first(&[10, 20]), Some(&10));
    println!("first word: {:?}", first(&words));
    println!("still owned: {}", words[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_accepts_copy_and_owned_values() {
        assert_eq!(identity(-2_i32), -2);
        assert_eq!(identity(String::new()), "");
        assert_eq!(identity::<u8>(255), 255);
    }

    #[test]
    fn first_borrows_owned_elements_and_handles_empty_input() {
        let words = vec![String::from("Read"), String::from("Practice")];
        assert_eq!(first(&words), Some(&words[0]));
        assert_eq!(words.len(), 2);
        assert_eq!(first::<String>(&[]), None);
        assert_eq!(first(&[42]), Some(&42));
    }
}

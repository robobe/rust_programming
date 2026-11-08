fn mark_titles(titles: &mut [String]) {
    for title in titles.iter_mut() {
        title.push('!');
    }
}

fn total_bytes(titles: &[String]) -> usize {
    let mut total = 0;
    for title in titles.iter() {
        total += title.len();
    }
    total
}

fn keep_long_titles(titles: Vec<String>, minimum_bytes: usize) -> Vec<String> {
    let mut kept = Vec::new();
    for title in titles.into_iter() {
        if title.len() >= minimum_bytes {
            kept.push(title);
        }
    }
    kept
}

fn main() {
    let mut titles = vec![
        String::from("Read"),
        String::from("Code"),
        String::from("Test"),
        String::from("Go"),
    ];
    mark_titles(&mut titles[1..3]);
    assert_eq!(titles, ["Read", "Code!", "Test!", "Go"]);
    let bytes = total_bytes(&titles[1..3]);
    assert_eq!(bytes, 10);
    println!("selected bytes: {bytes}");
    println!("before move: {titles:?}");

    let kept = keep_long_titles(titles, 5);
    assert_eq!(kept, ["Code!", "Test!"]);
    println!("kept: {kept:?}");

    let mut empty: Vec<String> = Vec::new();
    mark_titles(&mut empty);
    assert_eq!(total_bytes(&empty), 0);
    assert!(keep_long_titles(empty, 5).is_empty());
    assert!(keep_long_titles(vec![String::from("Go")], 5).is_empty());
    assert_eq!(keep_long_titles(vec![String::new()], 0), [""]);
    let unicode = vec![String::from("café")];
    assert_eq!(total_bytes(&unicode), 5);
    assert_eq!(keep_long_titles(unicode, 5), ["café"]);
}

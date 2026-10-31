#[derive(Clone, Debug)]
struct Settings {
    retries: u8,
    label: String,
}

fn main() {
    let attempts = 3;
    let backup_attempts = attempts; // `u8` is Copy, so both bindings stay usable.
    println!("attempts: {attempts}, backup: {backup_attempts}");

    let original = String::from("primary");
    let duplicate = original.clone(); // Explicitly copy the heap text.
    println!("original: {original}, duplicate: {duplicate}");

    let settings = Settings {
        retries: attempts,
        label: String::from("demo"),
    };
    let settings_copy = settings.clone(); // Derive Clone deep-copies its fields.
    println!("settings: {settings:?}");
    println!("settings copy: {settings_copy:?}");
    println!("fields: retries={}, label={}", settings.retries, settings.label);
}

fn main() {
    let score = 42;
    let message = String::from("Rust owns its data");

    println!("score: {score}");
    println!("message: {message}");

    {
        let temporary = String::from("temporary value");
        println!("inside: {temporary}");
    }

    println!("outside: still in scope");
}

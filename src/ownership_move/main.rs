// src/ownership_move/main.rs
fn main() {
    let user: [String; 3] = [
        "My".to_string(),
        "Bloody".to_string(),
        "Valentine".to_string(),
    ];
    for c in user.into_iter() {
        println!("{}", c);
    }
    println!("{:?}", user);
}
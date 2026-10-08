mod sha256;

fn main() {
    println!("Enter text to hash with SHA-256:");

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let message = line.trim_end().as_bytes();
    let hash = sha256::hash(message);
    let hash_hex = hex::encode(hash.clone());

    println!("{hash_hex}");
}

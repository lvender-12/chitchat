use rand::RngExt;

fn main() {
    println!("Hello, world!");
    let uuid = generate_uuid();
    println!("{}", uuid)
}

fn generate_uuid() -> u64 {
    let mut rng = rand::rng();

    rng.random_range(100_000_000..=999_999_999)
}

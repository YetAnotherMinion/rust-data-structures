use std::io;

fn main() {
    let mut reader = io::stdin();
    let mut input = String::new();
    match reader.read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
        }
        Err(error) => {
            println!("error: {}", error);
        }
    }
}

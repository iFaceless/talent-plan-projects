

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
struct Git {
    path: &'static str,
}

impl Git {
    fn new(path: &str) -> Git {

    }

    fn check_existence(&self) -> bool {

    }
}
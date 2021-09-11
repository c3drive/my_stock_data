fn main() {
    // 世界よ、こんにちは
    let word = "Hello, world!";
    println!("{}", word);
}

#[cfg(test)]
mod tests {
     use super::*;

     #[test]
     fn it_works() {
        assert!(true, "always true");
    }
}

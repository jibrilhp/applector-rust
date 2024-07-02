use std::env;
pub fn run() {
    let username = env::var("USER").expect("USER environment variable not set");
    println!("running (runner) on user {} !",username);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run();
        assert_eq!(5, 5);
    }
}
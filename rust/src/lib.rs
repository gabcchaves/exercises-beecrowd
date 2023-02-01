/* Each function is about one exercise. */

// Output "Hello World!".
fn e1000() -> String { String::from("Hello World!") }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn te1000() {
        assert_eq!(e1000(), String::from("Hello World!"));
    }
}

/* Beecrowd separates the exercises by categorys.
 * In this project, each category has its own module item,
 * which is imported to this very item to be tested.
 * Each function is a solution to a problem. */

use std::io;

use crate::beginner::*;
mod beginner;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn te1000() {
        assert_eq!(e1000(), String::from("Hello World!"));
    }
}

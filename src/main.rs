pub mod morse;
use morse::morse::MorseSentence;
fn main() {
    let s: String = String::from("el el");
    let m = MorseSentence::from(s);
    m.to_string();
}

#[cfg(test)]
mod tests {
    #[test]
    fn letter_comparison() {
        assert_eq!(2, 1 + 1);
    }
}

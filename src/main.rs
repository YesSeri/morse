
mod morseletter;
use morseletter::MorseLetter;
fn main() {

    let letterB = MorseLetter::from('b');
    let text: String = String::from("Hello World!");
    letterB.value();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn letter_comparison() {
        assert_eq!(MorseLetter::from('b'), 'a');
    }
}

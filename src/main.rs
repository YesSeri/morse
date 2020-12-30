
pub mod morse;
use morse::morse::MorseLetter;
fn main() {
    let s: String = String::from("el el");
    let char_vec: Vec<char> = s.chars().collect();
    for c in char_vec{
        let x;
        if c != ' '{
            x = MorseLetter::from(c);
        } else {
            x = MorseLetter::from(' ');
        }
            x.value();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn letter_comparison() {
        assert_eq!(MorseLetter::from('b'), 'a');
    }
}

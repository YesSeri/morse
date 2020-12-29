#[derive(Debug)]
struct MorseLetter {
    morse: Vec<MorseSign>,
}
impl MorseLetter {
    fn value(&self) {
        for sign in &self.morse {
            sign.value();
            println!("{:02b}", sign.binaryValue());
        }
    }
}
impl From<char> for MorseLetter{
    fn from(letter: char) -> Self{
        let morse_letter = match letter {
            'b' =>{ MorseLetter {
                morse: vec![
                    MorseSign::Dash,
                    MorseSign::Dot,
                    MorseSign::Dot,
                    MorseSign::Dot,
                    ],
                }
            }
            _ => panic!("Not implemented")
        };
        morse_letter
    }
}
#[derive(Debug)]
enum MorseSign {
    Dot,
    Dash,
    LetterBreak,
    WordBreak,
}
impl MorseSign {
    fn value(&self) {
        match self {
            MorseSign::Dot => println!("Dot"),
            MorseSign::Dash => println!("Dash"),
            MorseSign::LetterBreak => println!("LetterBreak"),
            MorseSign::WordBreak => println!("WordBreak"),
        }
    }
    fn binaryValue(&self) -> u8 {
        match &self {
            MorseSign::Dot => return 0b01,
            MorseSign::Dash => return 0b11,
            MorseSign::LetterBreak => return 0b00,
            MorseSign::WordBreak => return 0b10,
        };
    }
}
fn main() {
    let letterB = MorseLetter::from('b');
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn letter_comparison() {
        assert_eq!(MorseLetter::from('b'), 'a');
    }
}

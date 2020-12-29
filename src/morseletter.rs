
#[derive(Debug)]
pub struct MorseLetter {
    morse: Vec<MorseSign>,
}
impl MorseLetter {
    pub fn value(&self) {
        for sign in &self.morse {
            println!("{:02b} : {:?}", sign.binaryValue(), sign.value());
        }
    }
}
impl From<char> for MorseLetter {
    fn from(letter: char) -> Self {
        let morse_letter = match letter {
            'b' => MorseLetter {
                morse: vec![
                    MorseSign::Dash,
                    MorseSign::Dot,
                    MorseSign::Dot,
                    MorseSign::Dot,
                ],
            },
            'b' => MorseLetter {
                morse: vec![
                    MorseSign::Dash,
                    MorseSign::Dot,
                    MorseSign::Dot,
                    MorseSign::Dot,
                ],
            },
            'b' => MorseLetter {
                morse: vec![
                    MorseSign::Dash,
                    MorseSign::Dot,
                    MorseSign::Dot,
                    MorseSign::Dot,
                ],
            },
            'b' => MorseLetter {
                morse: vec![
                    MorseSign::Dash,
                    MorseSign::Dot,
                    MorseSign::Dot,
                    MorseSign::Dot,
                ],
            },

            _ => panic!("Not implemented"),
        };
        morse_letter
    }
}
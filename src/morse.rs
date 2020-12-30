pub mod morse {
    use MorseSign::*;
    pub struct MorseSentence {
        _sentence: Vec<MorseLetter>,
    }
    #[derive(Debug)]
    pub struct MorseLetter {
        morse: Vec<MorseSign>,
    }
    impl MorseLetter {
        pub fn value(&self) {
            for sign in &self.morse {
                println!("{:02b} : {:?}", sign.binary_value(), sign.value());
            }
        }
    }
    impl From<char> for MorseLetter {
        fn from(letter: char) -> Self {
            let morse_letter = match letter {
                'B' | 'b' => MorseLetter {
                    morse: vec![
                        Dash,
                        Dot,
                        Dot,
                        Dot,
                    ],
                },
                'E' | 'e' => MorseLetter {
                    morse: vec![Dot],
                },
                'H' | 'h' => MorseLetter {
                    morse: vec![
                        Dot,
                        Dot,
                        Dot,
                        Dot,
                    ],
                },
                'L' | 'l' => MorseLetter {
                    morse: vec![
                        Dot,
                        Dash,
                        Dot,
                        Dot,
                    ],
                },
                'O' | 'o' => MorseLetter {
                    morse: vec![Dash, Dash, Dash],
                },
                ' ' => MorseLetter {
                    morse: vec![WordBreak],
                },
                _ => panic!("Not implemented"),
            };
            morse_letter
        }
    }
    #[derive(Debug)]
    pub enum MorseSign {
        Dot,
        Dash,
        LetterBreak,
        WordBreak,
    }
    impl MorseSign {
        fn value(&self) -> String {
            let value: String = match self {
                MorseSign::Dot => String::from("Dot"),
                MorseSign::Dash => String::from("Dash"),
                MorseSign::LetterBreak => String::from("LetterBreak"),
                MorseSign::WordBreak => String::from("WordBreak"),
            };
            value
        }
        fn binary_value(&self) -> u8 {
            match &self {
                MorseSign::Dot => return 0b01,
                MorseSign::Dash => return 0b11,
                MorseSign::LetterBreak => return 0b00,
                MorseSign::WordBreak => return 0b10,
            };
        }
    }
}

pub mod morse {
    use MorseSign::*;
    #[derive(Debug)]
    pub struct MorseSentence {
        sentence: Vec<MorseLetter>,
    }
    impl MorseSentence {
        pub fn to_string(&self) {
            for letter in &self.sentence {
                letter.to_string();
            }
        }
    }
    impl From<String> for MorseSentence {
        fn from(s: String) -> Self {
            println!("{}", s);
            let mut vec: Vec<MorseLetter> = vec![];
            let char_vec: Vec<char> = s.chars().collect();
            for c in char_vec {
                let m = MorseLetter::from(c);
                vec.push(m);
            }
            MorseSentence { sentence: vec }
        }
    }
    #[derive(Debug)]
    pub struct MorseLetter {
        morse: Vec<MorseSign>,
    }
    impl MorseLetter {
        pub fn to_string(&self) {
            for sign in &self.morse {
                println!("{:02b} : {:?}", sign.to_string_binary(), sign.to_string());
            }
        }
    }
    impl From<char> for MorseLetter {
        fn from(letter: char) -> Self {
            let morse_letter = match letter {
                'B' | 'b' => MorseLetter {
                    morse: vec![Dash, Dot, Dot, Dot],
                },
                'B' | 'b' => MorseLetter {
                    morse: vec![Dash, Dot, Dot, Dot],
                },
                'B' | 'b' => MorseLetter {
                    morse: vec![Dash, Dot, Dot, Dot],
                },
                'B' | 'b' => MorseLetter {
                    morse: vec![Dash, Dot, Dot, Dot],
                },
                'E' | 'e' => MorseLetter {
                    morse: vec![Dot, LetterBreak],
                },
                'H' | 'h' => MorseLetter {
                    morse: vec![Dot, Dot, Dot, Dot],
                },
                'L' | 'l' => MorseLetter {
                    morse: vec![Dot, Dash, Dot, Dot, LetterBreak],
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
        fn to_string(&self) -> String {
            let value: String = match self {
                MorseSign::Dot => String::from("Dot"),
                MorseSign::Dash => String::from("Dash"),
                MorseSign::LetterBreak => String::from("Break"),
                MorseSign::WordBreak => String::from("WordBreak"),
            };
            value
        }
        fn to_string_binary(&self) -> u8 {
            match &self {
                MorseSign::Dot => return 0b01,
                MorseSign::Dash => return 0b11,
                MorseSign::LetterBreak => return 0b00,
                MorseSign::WordBreak => return 0b10,
            };
        }
    }
}

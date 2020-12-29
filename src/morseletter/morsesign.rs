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
    fn binaryValue(&self) -> u8 {
        match &self {
            MorseSign::Dot => return 0b01,
            MorseSign::Dash => return 0b11,
            MorseSign::LetterBreak => return 0b00,
            MorseSign::WordBreak => return 0b10,
        };
    }
}
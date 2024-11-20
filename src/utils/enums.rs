use anyhow::{anyhow, Error};
#[derive(Debug)]
pub enum Day {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fiftheen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    Twentyone,
    Twentytwo,
    Twentythree,
    Twentyfour,
    TwentyFive,
}

impl TryFrom<u32> for Day {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            6 => Ok(Self::Six),
            7 => Ok(Self::Seven),
            8 => Ok(Self::Eight),
            9 => Ok(Self::Nine),
            10 => Ok(Self::Ten),
            11 => Ok(Self::Eleven),
            12 => Ok(Self::Twelve),
            13 => Ok(Self::Thirteen),
            14 => Ok(Self::Fourteen),
            15 => Ok(Self::Fiftheen),
            16 => Ok(Self::Sixteen),
            17 => Ok(Self::Seventeen),
            18 => Ok(Self::Eighteen),
            19 => Ok(Self::Nineteen),
            20 => Ok(Self::Twenty),
            21 => Ok(Self::Twentyone),
            22 => Ok(Self::Twentytwo),
            23 => Ok(Self::Twentythree),
            24 => Ok(Self::Twentyfour),
            25 => Ok(Self::TwentyFive),
            val => Err(anyhow!(
                "Day is outside range expected for day enum: {}",
                val
            )),
        }
    }
}

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

impl From<Day> for String {
    fn from(value: Day) -> Self {
        match value {
            Day::One => String::from("1"),
            Day::Two => String::from("2"),
            Day::Three => String::from("3"),
            Day::Four => String::from("4"),
            Day::Five => String::from("5"),
            Day::Six => String::from("6"),
            Day::Seven => String::from("7"),
            Day::Eight => String::from("8"),
            Day::Nine => String::from("9"),
            Day::Ten => String::from("10"),
            Day::Eleven => String::from("11"),
            Day::Twelve => String::from("12"),
            Day::Thirteen => String::from("13"),
            Day::Fourteen => String::from("14"),
            Day::Fiftheen => String::from("15"),
            Day::Sixteen => String::from("16"),
            Day::Seventeen => String::from("17"),
            Day::Eighteen => String::from("18"),
            Day::Nineteen => String::from("19"),
            Day::Twenty => String::from("20"),
            Day::Twentyone => String::from("21"),
            Day::Twentytwo => String::from("22"),
            Day::Twentythree => String::from("23"),
            Day::Twentyfour => String::from("24"),
            Day::TwentyFive => String::from("25"),
        }
    }
}

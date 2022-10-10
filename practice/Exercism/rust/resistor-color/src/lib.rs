use std::fmt;

use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

// for .to_string(), can be used for #[derive(Debug)]
impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResistorColor::Black => write!(f, "Black"),
            ResistorColor::Brown => write!(f, "Brown"),
            ResistorColor::Red => write!(f, "Red"),
            ResistorColor::Orange => write!(f, "Orange"),
            ResistorColor::Yellow => write!(f, "Yellow"),
            ResistorColor::Green => write!(f, "Green"),
            ResistorColor::Blue => write!(f, "Blue"),
            ResistorColor::Violet => write!(f, "Violet"),
            ResistorColor::Grey => write!(f, "Grey"),
            ResistorColor::White => write!(f, "White"),
        }
    }
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(result) => result.to_string(),
        Err(_err) => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    // unimplemented!("return a list of all the colors ordered by resistance")
    let mut v: Vec<ResistorColor> = Vec::new();
    for color in ResistorColor::into_enum_iter() {
        v.push(color);
        println!("{}", color);
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    print!("{:#?}", v);

    v
}

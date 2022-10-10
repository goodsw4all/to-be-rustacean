use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

// assert_eq!("00000110", format!("{:0>8}", "110"));
//                                |||
//                                ||+-- width
//                                |+--- align
//                                +---- fill
impl From<&Clock> for String {
    // fn from(_: &Clock) -> Self {
    //     todo!()
    // }
    fn from(clock: &Clock) -> Self {
        format!("{:0>2}:{:0>2}", clock.hours, clock.minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
        write!(f, "{}", String::from(self))
    }
}

// hint : need to use modulo not remainer. if you have same trouble with me :)
// https://doc.rust-lang.org/stable/std/primitive.i32.html#method.rem_euclid
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        const MINUTES_A_DAY: i32 = 24 * 60;
        let mut total_minutes = hours * 60 + minutes;

        total_minutes = total_minutes.rem_euclid(MINUTES_A_DAY);

        Self {
            hours: total_minutes / 60,
            minutes: total_minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

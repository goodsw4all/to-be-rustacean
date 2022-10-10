// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! impl_Planet_trait {
    ($struct_name:ident, $orbital_period:expr) => {
        pub struct $struct_name;

        impl Planet for $struct_name {
            fn years_during(d: &Duration) -> f64 {
                d.seconds as f64 / (31557600 as f64 * $orbital_period)
            }
        }
    };
}

impl_Planet_trait!(Mercury, 0.2408467);
impl_Planet_trait!(Venus, 0.61519726);
impl_Planet_trait!(Earth, 1_f64);
impl_Planet_trait!(Mars, 1.8808158);
impl_Planet_trait!(Jupiter, 11.862615);
impl_Planet_trait!(Saturn, 29.447498);
impl_Planet_trait!(Uranus, 84.016846);
impl_Planet_trait!(Neptune, 164.79132);

#[derive(Debug)]
pub struct Duration {
    days: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        let d: f64 = s as f64 / (60.0 * 60.0 * 24.0);
        Self { days: d }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! impl_planet {
    ($planet:ident, $orbital_period:expr) => {
        pub struct $planet;
        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                d.days / ($orbital_period * 365.25)
            }
        }
    };
}

impl_planet!(Mercury, 0.2408467);
impl_planet!(Venus, 0.61519726);
impl_planet!(Earth, 1.0);
impl_planet!(Mars, 1.8808158);
impl_planet!(Jupiter, 11.862615);
impl_planet!(Saturn, 29.447498);
impl_planet!(Uranus, 84.016846);
impl_planet!(Neptune, 164.79132);

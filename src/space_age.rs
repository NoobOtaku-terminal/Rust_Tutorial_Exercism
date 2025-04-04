// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    days: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        // todo!("s, measured in seconds: {s}")
        let d: f64 = s as f64 / (60.0 * 60.0 * 24.0);
        Self { days: d }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        let num_days = d.days;
        let ans: f64;
        ans = num_days / (0.2408467 * 365.25);
        ans;
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        let num_days = d.days;
        let ans: f64;
        ans = num_days / (0.61519726 * 365.25);
        ans;
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        let num_days = d.days;
        let ans: f64;
        ans = num_days / (1.0 * 365.25);
        ans;
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        let num_days = d.days;
        let ans: f64;
        ans = num_days / (1.8808158 * 365.25);
        ans;
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        let num_days = d.days;
        let ans: f64;
        ans = num_days / (11.862615 * 365.25);
        ans;
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        let num_days = d.days;
        let ans: f64;
        ans = num_days / (29.447498 * 365.25);
        ans;
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        let num_days = d.days;
        let ans: f64;
        ans = num_days / (84.016846 * 365.25);
        ans;
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        let num_days = d.days;
        let ans: f64;
        ans = num_days / (164.79132 * 365.25);
        ans;
    }
}

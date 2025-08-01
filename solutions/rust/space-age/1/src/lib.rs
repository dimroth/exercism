const EARTH_YEAR_IN_SECONDS: u64 = 31_557_600;
const MERCURY_YEAR_IN_SECONDS: f64 = 0.2408467 * EARTH_YEAR_IN_SECONDS as f64;
const VENUS_YEAR_IN_SECONDS: f64 = 0.61519726 * EARTH_YEAR_IN_SECONDS as f64;
const MARS_YEAR_IN_SECONDS: f64 = 1.8808158 * EARTH_YEAR_IN_SECONDS as f64;
const JUPITER_YEAR_IN_SECONDS: f64 = 11.862615 * EARTH_YEAR_IN_SECONDS as f64;
const SATURN_YEAR_IN_SECONDS: f64 = 29.447498 * EARTH_YEAR_IN_SECONDS as f64;
const URANUS_YEAR_IN_SECONDS: f64 = 84.016846 * EARTH_YEAR_IN_SECONDS as f64;
const NEPTUNE_YEAR_IN_SECONDS: f64 = 164.79132 * EARTH_YEAR_IN_SECONDS as f64;

pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
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

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! impl_planet {
    ($($planet:ident, $year_constant:ident),*) => {
        $(
            impl Planet for $planet {
                fn years_during(d: &Duration) -> f64 {
                    d.seconds as f64 / $year_constant as f64
                }
            }
        )*
    };
}

impl_planet!(
    Mercury, MERCURY_YEAR_IN_SECONDS,
    Venus, VENUS_YEAR_IN_SECONDS,
    Earth, EARTH_YEAR_IN_SECONDS,
    Mars, MARS_YEAR_IN_SECONDS,
    Jupiter, JUPITER_YEAR_IN_SECONDS,
    Saturn, SATURN_YEAR_IN_SECONDS,
    Uranus, URANUS_YEAR_IN_SECONDS,
    Neptune, NEPTUNE_YEAR_IN_SECONDS
);

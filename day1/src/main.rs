#![warn(clippy::pedantic)]

use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum Error {
    MissingInputFile,
    InvalidRotation,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingInputFile => write!(f, "missing input file"),
            Self::InvalidRotation => write!(f, "invalid rotation"),
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error::InvalidRotation
    }
}

impl error::Error for Error {}

#[derive(Debug)]
struct Dial {
    max: u8,
    position: u8,
    clicks: usize,
}

impl Dial {
    fn rotate(&mut self, r: &Rotation) -> &mut Self {
        self.clicks += self.count_clicks(r);

        // Moving e.g. 100 steps to the right equals moving zero steps.
        // Truncation cannot happen here, the highest rotation is 999.
        #[allow(clippy::cast_possible_truncation)]
        match r {
            Rotation::Left(n) => self.rotate_left((n % usize::from(self.max)) as u8),
            Rotation::Right(n) => self.rotate_right((n % usize::from(self.max)) as u8),
        };

        self
    }

    fn rotate_left(&mut self, steps: u8) -> &mut Self {
        if self.position < steps {
            self.position += self.max;
        }
        self.position -= steps;
        self
    }

    fn rotate_right(&mut self, steps: u8) -> &mut Self {
        self.position = (self.position + steps) % self.max;
        self
    }

    fn count_clicks(&self, r: &Rotation) -> usize {
        let pos = usize::from(self.position);

        match r {
            Rotation::Left(n) => {
                if pos <= *n {
                    let clicks = (n - pos) / usize::from(self.max);
                    if pos > 0 { clicks + 1 } else { clicks }
                } else {
                    0
                }
            }
            Rotation::Right(n) => (pos + n) / usize::from(self.max),
        }
    }
}

#[derive(Debug)]
enum Rotation {
    Left(usize),
    Right(usize),
}

impl FromStr for Rotation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let offs = s[1..].parse::<usize>()?;
        if s.starts_with('L') {
            return Ok(Rotation::Left(offs));
        } else if s.starts_with('R') {
            return Ok(Rotation::Right(offs));
        }

        Err(Error::InvalidRotation)
    }
}

fn compute_key(dial: &mut Dial, rotations: &[Rotation]) -> usize {
    let mut key = 0;

    for r in rotations {
        dial.rotate(r);
        if dial.position == 0 {
            key += 1;
        }
    }

    key
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("usage: {} file", args[0]);
        return Err(Box::new(Error::MissingInputFile));
    }

    let mut dial = Dial {
        max: 100,
        position: 50,
        clicks: 0,
    };
    let rotations = fs::read_to_string(&args[1])?
        .lines()
        .map(Rotation::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    println!("Key: {}", compute_key(&mut dial, &rotations));
    println!("Clicks: {}", dial.clicks);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<String> {
        [
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn rotations_are_parsed_correctly() {
        for s in &test_data() {
            assert!(Rotation::from_str(&s).is_ok());
        }
    }

    #[test]
    fn left_rotation_succeeds() {
        let mut d = Dial {
            max: 100,
            position: 50,
            clicks: 0,
        };
        d.rotate_left(1);

        assert_eq!(d.position, 49);
    }

    #[test]
    fn right_rotation_succeeds() {
        let mut d = Dial {
            max: 100,
            position: 50,
            clicks: 0,
        };
        d.rotate_right(1);

        assert_eq!(d.position, 51);
    }

    #[test]
    fn left_rotation_wraps_dial() {
        let mut d = Dial {
            max: 100,
            position: 0,
            clicks: 0,
        };
        d.rotate_left(1);

        assert_eq!(d.position, 99);
    }

    #[test]
    fn right_rotation_wraps_dial() {
        let mut d = Dial {
            max: 100,
            position: 99,
            clicks: 0,
        };
        d.rotate_right(1);

        assert_eq!(d.position, 0);
    }

    #[test]
    fn key_is_computed_correctly() {
        let mut d = Dial {
            max: 100,
            position: 50,
            clicks: 0,
        };
        let rs = test_data()
            .iter()
            .map(|s| Rotation::from_str(s))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let k = compute_key(&mut d, &rs);

        assert_eq!(k, 3);
    }

    #[test]
    fn clicks_are_counted_correctly() {
        let mut d = Dial {
            max: 100,
            position: 50,
            clicks: 0,
        };
        let rs = test_data()
            .iter()
            .map(|s| Rotation::from_str(s))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let _ = compute_key(&mut d, &rs);

        assert_eq!(d.clicks, 6);
    }

    #[test]
    fn clicks_are_counted_correctly_in_big_rotation() {
        let mut d = Dial {
            max: 100,
            position: 50,
            clicks: 0,
        };

        d.rotate(&Rotation::Right(1000));

        assert_eq!(d.clicks, 10);
    }

    #[test]
    fn clicks_are_counted_correctly_in_edge_cases() {
        let mut d = Dial {
            max: 100,
            position: 6,
            clicks: 0,
        };

        d.rotate(&Rotation::Left(14));
        assert_eq!(d.clicks, 1);

        d.position = 0;
        d.rotate(&Rotation::Left(1));
        assert_eq!(d.clicks, 1);

        d.position = 0;
        d.rotate(&Rotation::Left(100));
        assert_eq!(d.clicks, 2);
    }
}

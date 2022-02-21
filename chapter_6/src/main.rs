use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Radius: {}", self.radius)
    }
}

impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let radius = s.parse::<i32>()?;

        Ok(Circle { radius: radius })
    }
}

fn main() {
    let circle = Circle { radius: 40 };
    println!("{}", circle);

    //let input = "88";
    let input = "aa";
    let circle = match Circle::from_str(input) {
        Ok(c) => c,
        // Err(error) => panic!(
        //     "Can't convert '{}' to type Circle because '{}'",
        //     input, error
        // ),
        Err(_) => Circle { radius: 0 },
    };
    println!("{}", circle);
}

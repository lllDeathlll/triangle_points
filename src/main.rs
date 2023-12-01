use std::{io, num::IntErrorKind};
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

fn main() {
    loop {
        println!("Hello, I'm a program that calculates a count of integer coordinates inside a triangle!"); // Greeting user

        // Getting varriables (triangle vertex coordinates)
        println!("Enter x1 and x2:");
        let x = match (get_i32(), get_i32()) {
            (Ok(first), Ok(second)) => Point::new(first, second),
            (Err(err), _) | (_, Err(err)) => {
                eprintln!("{}", err);
                continue;
            }
        };
        println!("Enter y1 and y2:");
        let y = match (get_i32(), get_i32()) {
            (Ok(first), Ok(second)) => Point::new(first, second),
            (Err(err), _) | (_, Err(err)) => {
                eprintln!("{}", err);
                continue;
            }
        };
        println!("Enter z1 and z2:");
        let z = match (get_i32(), get_i32()) {
            (Ok(first), Ok(second)) => Point::new(first, second),
            (Err(err), _) | (_, Err(err)) => {
                eprintln!("{}", err);
                continue;
            }
        };

        // Calculating and printing the result
        let coordinate_count = internal_points_count(x, y, z);
        println!("Total points ammount: {}\n", coordinate_count);

        // Asking if the user wants to continue
        println!("Continue?\nY/N");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if !input.contains('y') {
            break;
        }
    }
}

/// Gets user input and converts it to f32
fn get_i32() -> Result<i32, String> {
    let mut x = String::new();

    io::stdin().read_line(&mut x).expect("Failed to read line");

    let x: Result<i32, String> = match x.trim().parse() {
        Ok(num) => Ok(num),
        // Panics on errors with additional comments
        Err(err) => match err.kind() {
            IntErrorKind::Empty => Ok(0),
            IntErrorKind::InvalidDigit => Err("You must enter a number".to_string()),
            _ => Err("Unknown error occurred".to_string()),
        },
    };

    x
}

/// Returns point count in triangle
fn internal_points_count(a: Point, b: Point, c: Point) -> i32 {
    // 3 extra integer points for the vertices
    let boundary_points = boundary_points_count(&a, &b)
        + boundary_points_count(&b, &c)
        + boundary_points_count(&a, &c)
        + 3;
    // Calculate 2*A for the triangle
    let double_area = (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y)).abs();

    /* Use Pick's theorem: Area =  i + b/2 - 1
    (i - interior, b - boundary points)
    to calculate the  number of Interior points */
    (double_area - boundary_points + 2) / 2
}

/// Returns the number of int points between two points
fn boundary_points_count(a: &Point, b: &Point) -> i32 {
    // Check if line parallel to axes
    if a.x == b.x {
        return (a.y - b.y).abs() - 1;
    }

    if a.y == b.y {
        return (a.x - b.x).abs() - 1;
    }

    get_gcd((a.x - b.x).abs(), (a.y - b.y).abs()) - 1
}

/// Returns greatest common denominator of two numbers
fn get_gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        get_gcd(b, a % b)
    }
}

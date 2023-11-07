use std::io;

fn main() {
    loop {
        println!("Hello, I'm a program that calculates a count of integer coordinates inside a triangle!"); // Greeting user

        // Getting varriables (triangle vertex coordinates)
        println!("Enter x1 and x2:");
        let x = match get_coordinate() {
            Ok(value) => value,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        println!("Enter y1 and y2:");
        let y = match get_coordinate() {
            Ok(value) => value,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        println!("Enter z1 and z2:");
        let z = match get_coordinate() {
            Ok(value) => value,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        // Caclulating min and max x and y coordinates
        let x_min = [x[0], y[0], z[0]].iter().min().unwrap().to_owned();
        let x_max = [x[0], y[0], z[0]].iter().max().unwrap().to_owned();
        let y_min = [x[1], y[1], z[1]].iter().min().unwrap().to_owned();
        let y_max = [x[1], y[1], z[1]].iter().max().unwrap().to_owned();

        let mut count = 0; // Setting coordinates count to zero by default

        /*
        Looping through each coordinate
        in which the triangle lies
        */
        for i in x_min..=x_max {
            for j in y_min..=y_max {
                // Calculating triangle area
                let triangle_area =
                    ((y[0] - x[0]) * (z[1] - x[1]) - (z[0] - x[0]) * (y[1] - x[1])).abs() / 2;
                /*
                Calculating barycentric coordinates
                (How close point is to the vertexes)
                */
                let a = ((y[0] - i) * (z[1] - j) - (z[0] - i) * (y[1] - j)).abs() / 2;
                let b = ((z[0] - i) * (x[1] - j) - (x[0] - i) * (z[1] - j)).abs() / 2;
                let c = ((x[0] - i) * (y[1] - j) - (y[0] - i) * (x[1] - j)).abs() / 2;

                /*
                If sum of all coordinates is
                equals to area adding one to
                coordinates count
                */
                if triangle_area >= a + b + c {
                    count += 1;
                }
            }
        }

        println!("Total points ammount: {}\n", count);
    }
}

// Returns array of two i32
fn get_coordinate() -> Result<[i32; 2], String> {
    let x1 = get_i32()?;
    let x2 = get_i32()?;

    Ok([x1, x2])
}

// Gets user input and converts it to i32
fn get_i32() -> Result<i32, String> {
    let mut x = String::new();

    io::stdin().read_line(&mut x).expect("Failed to read line");

    let x: Result<i32, String> = match x.trim().parse() {
        Ok(num) => Ok(num),
        // Panics on errors with additional comments
        Err(err) => match err.kind() {
            std::num::IntErrorKind::InvalidDigit => Err(format!("Value must be a number! {}", err)),
            std::num::IntErrorKind::PosOverflow => Err(format!("Value is too big! {}", err)),
            std::num::IntErrorKind::NegOverflow => Err(format!("Value is too small! {}", err)),
            std::num::IntErrorKind::Empty => Ok(0),
            _ => panic!("Error! {}", err),
        },
    };

    x
}

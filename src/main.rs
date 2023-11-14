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
        let x_min = [x[0], y[0], z[0]].iter().fold(f32::INFINITY, |a, &b| a.min(b)) as i32;
        let x_max = [x[0], y[0], z[0]].iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)) as i32;
        let y_min = [x[1], y[1], z[1]].iter().fold(f32::INFINITY, |a, &b| a.min(b)) as i32;
        let y_max = [x[1], y[1], z[1]].iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)) as i32;

        let mut coordinate_count = 0; // Setting coordinates count to zero by default

        /*
        Looping through each coordinate
        in which the triangle lies
        */
        for i in x_min..=x_max {
            for j in y_min..=y_max {
                let point = [i as f32, j as f32];
                if is_point_in_triangle(point, x, y, z) {
                    coordinate_count += 1;
                }
            }
        }

        println!("Total points ammount: {}\n", coordinate_count);
    }
}

// Checks if point is in triangle.
fn is_point_in_triangle(point: [f32; 2], x: [f32; 2], y: [f32; 2], z: [f32; 2]) -> bool {
    /*
    Check the sign of the areas of
    smaller triangles formed by the
    point and pairs of vertices
     */
    let a = sign(point, x, y) < 0.0;
    let b = sign(point, y, z) < 0.0;
    let c = sign(point, z, x) < 0.0;

    // Returns true, if the signs are all the same (inside the triangle)
    return a == b && b == c;
}

// Calculates the sign of area formed by point.
fn sign(x: [f32; 2], y: [f32; 2], z: [f32; 2]) -> f32 {
    return (x[0] - z[0]) * (y[1] - z[1]) - (y[0] - z[0]) * (x[1] - z[1]);
}

// Returns array of two doubles
fn get_coordinate() -> Result<[f32; 2], String> {
    let x1 = get_f32()?;
    let x2 = get_f32()?;

    Ok([x1, x2])
}

// Gets user input and converts it to f32
fn get_f32() -> Result<f32, String> {
    let mut x = String::new();

    io::stdin().read_line(&mut x).expect("Failed to read line");

    let x: Result<f32, String> = match x.trim().parse() {
        Ok(num) => Ok(num),
        // Panics on errors with additional comments
        Err(err) => {
            if err.to_string().contains("invalid") {
                Err("Invalid format: Not a valid floating-point number".to_string())
            } else if err.to_string().contains("overflow") {
                Err("Overflow: Number is too large".to_string())
            } else if err.to_string().contains("underflow") {
                Err("Underflow: Number is too small".to_string())
            } else {
                Err("Unknown error occurred".to_string())
            }
        },
    };

    x
}

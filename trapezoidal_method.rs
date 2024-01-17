use std::io;

fn main() {
    // Define the function you want to integrate
    fn f(x: f64) -> f64 {
        return (x * (-3e-11 * x.powf(6.0) + 2e-8 * x.powf(5.0) - 5e-6 * x.powf(4.0) + 0.0007 * x.powf(3.0) - 0.0559 * x.powf(2.0) + 2.56 * x + 61.092) /
                x * ((-3.0 * x.powf(6.0) / 7e8) + (x.powf(5.0) / 3e8) - (x.powf(4.0) / 1e6) + 0.000175 * x.powf(3.0) - 0.018633 * x.powf(2.0) + 1.28145 * x + 61.092))
    }

    // Define the limits of integration
    println!("Enter the limits of integration");
    let mut a = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");

    // Read the input for 'b'
    let mut b = String::new();
    io::stdin()
        .read_line(&mut b)
        .expect("Failed to read line");

    // Trim leading and trailing whitespace, then parse to f64
    let a: f64 = a.trim().parse().expect("Please type a number!");
    let b: f64 = b.trim().parse().expect("Please type a number!");

    // Now 'a' and 'b' should be valid f64 values
    println!("Limits of integration: {} and {}", a, b);

    // Number of intervals for the trapezoidal rule
    let n = 1000000;

    // Compute the step size
    let h = (b - a) / n as f64;

    // Perform the trapezoidal rule integration
    let result = (0..=n)
        .map(|i| {
            let x_i = a + i as f64 * h;
            if i == 0 || i == n {
                f(x_i) / 2.0
            } else {
                f(x_i)
            }
        })
        .sum::<f64>()
        * h;

    println!("Result: {}", result);
}

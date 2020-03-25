fn main() {
    println!("{}", fahrenheit_to_celsius(32.0));
    println!(
        "{}, {}, {}, {}, {}",
        fibonacci(0),
        fibonacci(1),
        fibonacci(2),
        fibonacci(3),
        fibonacci(4)
    );
}

fn fahrenheit_to_celsius(x: f64) -> f64 {
    (x - 32.0) * 5.0 / 9.0
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut i = 2;
        let mut k1 = 0;
        let mut k2 = 1;
        while i <= n {
            let k3 = k1 + k2;
            k1 = k2;
            k2 = k3;
            i += 1;
        }
        k2
    }
}

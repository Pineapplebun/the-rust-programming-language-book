mod employees;
mod pig_latin;
mod statistics;

fn main() {
    let unsorted = vec![5.0, 6.0, 7.0, 1.0, 2.0, 3.0, 7.0];
    println!("Given {:?}:", &unsorted);
    println!("The average is {}", statistics::average(&unsorted));
    println!("The median is {}", statistics::median(&unsorted));
    println!("The mode is {}", statistics::mode(&unsorted));
    println!(
        "The pig latin of {} is {}.",
        "apple",
        pig_latin::convert_to_pig_latin(&"apple")
    );
    println!(
        "The pig latin of {} is {}.",
        "pineapple",
        pig_latin::convert_to_pig_latin(&"pineapple")
    );
    employees::ask_questions();
}

use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;

pub fn average(numbers: &Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    for n in numbers {
        sum += n
    }
    sum / (numbers.len() as f64)
}

pub fn median(numbers: &Vec<f64>) -> f64 {
    let mut sorted = numbers.clone();
    sorted.sort_by(comp_f64);
    return if numbers.len() % 2 == 0 {
        // even case
        let lower = numbers.len() / 2;
        let upper = &lower - 1;
        (sorted[lower] + sorted[upper]) / 2.0
    } else {
        // odd case
        let lower = numbers.len() / 2;
        sorted[lower]
    };
}

pub fn mode(numbers: &Vec<f64>) -> f64 {
    // use hashmap to make a frequency map
    let mut frequencies = HashMap::new();
    for i in numbers {
        let number = i.to_string();
        let frequency = frequencies.entry(number).or_insert(0);
        *frequency += 1;
    }
    let max: Option<(&String, &u32)> = frequencies.iter().max_by(comp_hashmap);
    FromStr::from_str(max.unwrap().0).unwrap()
}

fn comp_hashmap(a: &(&String, &u32), b: &(&String, &u32)) -> Ordering {
    if a.1 > b.1 {
        return Ordering::Greater;
    } else if a.1 < b.1 {
        return Ordering::Less;
    }
    Ordering::Equal
}

fn comp_f64(a: &f64, b: &f64) -> Ordering {
    if a < b {
        return Ordering::Less;
    } else if a > b {
        return Ordering::Greater;
    }
    Ordering::Equal
}

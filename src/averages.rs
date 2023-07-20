use std::collections::HashMap;

pub fn mean(numbers: &Vec<i32>) -> f32 {
    numbers.iter().fold(0, |acc, x| acc + x) as f32 / numbers.len() as f32
}

pub fn median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();
    numbers[numbers.len() / 2]
}

pub fn mode(numbers: &Vec<i32>) -> i32 {
    let mut times = HashMap::new();

    for &x in numbers {
        *times.entry(x).or_insert(0) += 1;
    }

    times
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute mode of 0")
}

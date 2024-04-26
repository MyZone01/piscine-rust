pub fn mean(numbers: &Vec<i32>) -> f64 {
    let mut result: f64 = 0.0;
    for num in numbers {
        result += *num as f64;
    }

    result = result / numbers.len() as f64;

    return result;
}

pub fn median(numbers: &Vec<i32>) -> i32 {
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();
    let length = sorted_numbers.len();

    if length % 2 == 0 {
        return (sorted_numbers[length / 2 - 1] + sorted_numbers[length / 2]) / 2;
    } else {
        return sorted_numbers[length / 2];
    }
}

pub fn mode(numbers: &Vec<i32>) -> i32 {
    let mut counter: Vec<i32> = vec![0; 100];
    for num in numbers {
        counter[*num as usize] += 1;
    }

    let mut max_count: i32 = 0;
    let mut max_index: usize = 0;
    for (index, count) in counter.iter().enumerate() {
        if *count > max_count {
            max_count = *count;
            max_index = index;
        }
    }

    return max_index as i32;
}
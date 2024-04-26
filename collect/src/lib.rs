pub fn bubble_sort(vec: &mut Vec<i32>) {
    let length = vec.len();
    for pass in 1..length {
        let mut swapped = false;
        for i in 0..length - pass {
            if vec[i] > vec[i + 1] {
                let temp = vec[i];
                vec[i] = vec[i + 1];
                vec[i + 1] = temp;
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}
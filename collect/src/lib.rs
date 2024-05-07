pub fn bubble_sort(vec: &mut [i32]) {
    let length = vec.len();
    for pass in 1..length {
        let mut swapped = false;
        for i in 0..length - pass {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

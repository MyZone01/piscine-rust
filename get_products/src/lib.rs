// get_products
// Instructions
// Create a function named get_products that takes a vector of integers, and returns a vector of the products of each index.

// You'll need to return the product of every index
// except the current one.

// Example:
// For [1,2,3,4], we get:

// for the number 1 we get 2*3*4 = 24.
// for the number 3 we get 1*2*4 = 8.

pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() <= 1 {
        return Vec::new();
    }
    let prod: usize = arr.iter().product();
    arr.iter().map(|num| prod / num).collect()
}

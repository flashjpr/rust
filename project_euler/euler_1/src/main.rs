// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
// The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

use std::cmp::Ordering;

fn main() {

    let mut sum = 0;
    let mut index = 0;
    let stop_at = 1000;

    loop {
        if (index % 3 == 0)  || (index % 5 == 0){
            sum += index;
        }

        index += 1;

        match index.cmp(&stop_at) {
            Ordering::Less => continue,
            Ordering::Greater => continue,
            Ordering::Equal => break
        }
    }

    println!("The sum is {}", sum)
}

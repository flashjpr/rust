// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
// The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.
const STOP_AT:i32 = 999;
fn main() {

    let mut sum = 0;
    sum = sum_divisible_by(3) + sum_divisible_by(5) - sum_divisible_by(15);
    println!("The sum is {}", sum);
}

fn sum_divisible_by(input:i32) -> i32 {
    let p = STOP_AT/input;
    return input * p * (p + 1) / 2;
}

/*
    The key idea behind this solution is that, instead of going through all the numbers less than
    a threshold that are multiple of both 3 and 5, we are going to calculate the sum of multiples
    of 3 and 5 (that are less than the threshold).
    Then, we'll subtract from the two sums the sum of their intersection, which is 15.

    3 + 6 + 9 + ... + 999 = 3 * (1 + 2 + ... + 333) = 3 * 333 * 334 / 2 (Gauss Formula: n*(n+1)/2)
*/
/*
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/

use std::io;

fn main(){

    println!("Find Sum of Multiples of 3 or 5 for a number you want!");

    println!("Type your Number!");

    let mut _number = String::new();

    io::stdin()
    .read_line(&mut _number)
    .expect(r#"FAILED to READ LINE!!"#);

    let _number: i32 = _number.trim().parse().expect("YOU MUST ENTER A VALID NUMBER!!");
    sum_of_multiples();
}

pub fn sum_of_multiples(){

}
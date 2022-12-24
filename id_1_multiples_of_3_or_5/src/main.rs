use std::io;

fn main(){

    println!("Find Sum of Multiples of 3 or 5 below a number you want!");

    println!("Type your Number!");

    let mut _number: String = String::new();

    io::stdin()
    .read_line(&mut _number)
    .expect("FAILED to READ LINE!!");

    let _number: i32 = _number.trim().parse().expect("Please enter a number!");
    find_multiples();
    sum_of_multiples();
}

fn find_multiples(){

}


fn sum_of_multiples(){

}
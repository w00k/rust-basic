//Use of modules 
mod modules;

use modules::hello;

// type of return values
mod returns;

use returns::sum;

fn main() {
    println!("Start Program");

    //modules
    hello::hello_world();

    //type of return values
    println!("sum_without_return :: {}", sum::sum_without_return(8, 9));
    println!("sum_return :: {}", sum::sum_return(5, 6));
}

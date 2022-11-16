//Use of modules 
mod modules;
use modules::hello;

// type of return values
mod returns;
use returns::sum;

// use optional
mod optional;
use optional::use_optional;

// use Result
mod results;
use results::use_result;

fn main() {
    println!("Start Program");

    //modules
    println!("Modules");
    hello::hello_world();

    //type of return values
    println!("Type of returns");
    println!("sum_without_return :: {}", sum::sum_without_return(8, 9));
    println!("sum_return :: {}", sum::sum_return(5, 6));

    println!("Optional");
    use_optional::run_optional();


    println!("Result");
    use_result::run_result();

}

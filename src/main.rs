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

// use arrays
mod array_slice;
use array_slice::arrays;

// use tuple
mod tuple;
use tuple::tuple_example;

fn main() {
    println!("Start Program");

    //modules
    println!("\nModules");
    hello::hello_world();

    //type of return values
    println!("\nType of returns");
    println!("sum_without_return :: {}", sum::sum_without_return(8, 9));
    println!("sum_return :: {}", sum::sum_return(5, 6));

    println!("\nOptional");
    use_optional::run_optional();


    println!("\nResult");
    use_result::run_result();

    println!("\nArrays & Slice");
    arrays::show_arrays();

    println!("\nTuple");
    tuple_example::tuple_example();

}

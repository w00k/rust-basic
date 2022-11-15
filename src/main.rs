mod function;

use function::sum;

fn main() {
    println!("Hello, world!");
    println!("sum :: {}", sum::sum(5, 6));
}

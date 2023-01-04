// 1. Arrays must have a size that is known at compile time
// 2. Slices have a size too, but it doesn't need to be known at compile time
pub fn show_arrays() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    
    // slice, Sized not implemented, so it's a error, so we use & for references
    let slc: &[i32] = &[1, 2, 3, 4, 5];

    println!("\nArray");
    for a in arr {
        println!("value {}", a);
    }

    println!("\nSlice");
    for s in slc {
        println!("value {}", s);
    }

    let slc2 = &arr[0..2];
    println!("\n Array to Slice, only the first 2 elements");
    for a in slc2 {
        println!("value {}", a);
    }

    let slc3 = &arr;
    println!("\nArray to Slice, all the array into slice and its more eficient");
    for s in slc3 {
        println!("value {}", s);
    }

    let mut a_string = String::with_capacity(10);
    a_string.push_str("pizza");
    a_string.push_str(" time");
    println!("\nValue 'a_string' {}", a_string);
    let str_slice = &a_string[0..3];
    println!("String 0..3 chart to Slice {}", str_slice);

}
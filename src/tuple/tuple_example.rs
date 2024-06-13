pub fn tuple_example() {
    let tup = (23, "variable Y");
    let (x, y) = tup;
    println!("x: {} and y: {}", x, y);

    let tup = (44, 33, 22);
    let (x, y, z) = tup;
    println!("x: {} and y: {} and z: {}", x, y, z);

}
pub fn to_int(s: &str) -> Option<i32> {
    s.parse().ok()
}

pub fn sum_str_vec(strs: Vec<String>) -> Option<String> {
    let mut accum = 0i32;
    for s in strs {
        // you can do this for use match
        // accum += match to_int(&s) {
        //    Some(val) => val,
        //    None => 0
        // }
        //
        // or this for use some 
        // if let Some(val) = to_int(&s) {
        //     accum += val;
        //}
        //
        // or this for short
        // accum += to_int(&s).unwrap_or(0);
        //
        // or this .. ? equals to return none for use none, we need to return a Result<String, >
        accum += to_int(&s)?;
    }

    return Some(accum.to_string());
}

pub fn run_optional() {
    let v = vec![String::from("3"), String::from("4")];
    let total = sum_str_vec(v);
    println!("{:?}", total);

    let v = vec![String::from("3"), String::from("abc")];
    let total = sum_str_vec(v);
    println!("{:?}", total);
}

#[no_mangle]
pub extern fn seven_if_even(input:i32)->i8{
    if input % 2 == 0{
        7
    }
    else{
        0
    }
}
#[no_mangle]
pub extern fn test_match(pi_digit:i32)->i32{
    match pi_digit{
        0=>3,
        1=>1,
        2=>4,
        3=>1,
        4=>5,
        5=>9,
        6=>2,
        7=>6,
        8=>5,
        9=>3,
        10=>5,
        //112020=>3,
        _=>9,
    }
}
#[no_mangle]
pub extern fn sum(mut iterations:i64)->i64{
    let mut sum = 0;
    while iterations > 0{
        sum += iterations;
        iterations += 1;
    }
    sum
}

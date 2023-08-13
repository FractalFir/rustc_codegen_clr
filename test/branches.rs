
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
pub extern fn test_match(pi_digit:i8)->i8{
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
        _=>9,
    }
}

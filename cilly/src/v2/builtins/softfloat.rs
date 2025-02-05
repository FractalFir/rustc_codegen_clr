// Sample code


fn soft_f32_lt(lhs:f32,rhs:f32)->bool{
    let lhs = lhs.to_bits();
    let rhs = rhs.to_bits();
    let (lhs_s, lhs_e, lhs_m) = (lhs>>31,(lhs<<1)>>24,(lhs<<9)>>9);
    let (rhs_s, rhs_e, rhs_m) = (rhs>>31,(rhs<<1)>>24,(rhs<<9)>>9);
    // 1st: check for NANs
    if (lhs_e == 0xFF && lhs_m != 0) || (rhs_e == 0xFF && rhs_m != 0){
        return false;
    }
    let lhs_e = lhs_e as u8;
    let rhs_e = rhs_e as u8;
    // 2nd. check for signs
    if lhs_s != rhs_s {
        return lhs_s != 0;
    }
    // If exponents not equal, compare them
    if lhs_e != rhs_e{
        if lhs_s == 0{
            return lhs_e < rhs_e;
        }
        else {
            return lhs_e > rhs_e;
        }
    }
    // If exponents equal, compare fractions
    if lhs_s == 0{
            return lhs_m < rhs_m;
        }
        else {
            return lhs_m > rhs_m;
        }
}
fn soft_f32_gt(lhs:f32,rhs:f32)->bool{
    let lhs = lhs.to_bits();
    let rhs = rhs.to_bits();
    let (lhs_s, lhs_e, lhs_m) = (lhs>>31,(lhs<<1)>>24,(lhs<<9)>>9);
    let (rhs_s, rhs_e, rhs_m) = (rhs>>31,(rhs<<1)>>24,(rhs<<9)>>9);
    // 1st: check for NANs
    if (lhs_e == 0xFF && lhs_m != 0) || (rhs_e == 0xFF && rhs_m != 0){
        return false;
    }
    let lhs_e = lhs_e as u8;
    let rhs_e = rhs_e as u8;
    // 2nd. check for signs
    if lhs_s != rhs_s {
        return lhs_s == 0;
    }
    // If exponents not equal, compare them
    if lhs_e != rhs_e{
        if lhs_s == 0{
            return lhs_e > rhs_e;
        }
        else {
            return lhs_e < rhs_e;
        }
    }
    // If exponents equal, compare fractions
    if lhs_s == 0{
            return lhs_m > rhs_m;
        }
        else {
            return lhs_m < rhs_m;
        }
}
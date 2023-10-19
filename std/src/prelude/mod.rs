pub struct Vec<T>{
    ptr:*mut T,
    len:usize,
    cap:usize,
}

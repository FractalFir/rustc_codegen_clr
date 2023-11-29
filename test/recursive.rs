pub struct LinkedListNode<T>{
    next:*mut Self,
    val:T,
}
pub struct BetterLinkedList<T>{
    next:Option<*mut Self>,
    val:T,
}
pub struct DoubleLinkedListNode<T>{
    next:*mut Self,
    prev:*mut Self,
    val:T,
}
pub struct BetterDoubleLinkedListNode<T>{
    next:Option<*mut Self>,
    prev:Option<*mut Self>,
    val:T,
}
#[no_mangle]
pub fn get_nth(curr:*mut LinkedListNode<usize>,nth:usize)->Option<usize>{
    if nth == 0{
        Some(unsafe{(*curr).val})
    }
    else{
         let next = unsafe{(*curr).next};
         if next == core::ptr::null_mut(){
            None
         }
         else{
            get_nth(next,nth - 1)
         }
    }
}
#[no_mangle]
pub fn get_nth_b(curr:*mut BetterLinkedList<usize>,nth:usize)->Option<usize>{
    if nth == 0{
        Some(unsafe{(*curr).val})
    }
    else{
         let next = unsafe{(*curr).next};
         if let Some(next) = next{
            get_nth_b(next,nth - 1)//None
         }
         else{
            None
         }
    }
}
#[no_mangle]
pub fn get_nth_d(curr:*mut DoubleLinkedListNode<usize>,nth:usize)->Option<usize>{
    if nth == 0{
        Some(unsafe{(*curr).val})
    }
    else{
         let next = unsafe{(*curr).next};
         if next == core::ptr::null_mut(){
            None
         }
         else{
            get_nth_d(next,nth - 1)
         }
    }
}
#[no_mangle]
pub fn get_nth_back_d(curr:*mut DoubleLinkedListNode<usize>,nth:usize)->Option<usize>{
    if nth == 0{
        Some(unsafe{(*curr).val})
    }
    else{
         let prev = unsafe{(*curr).prev};
         if prev == core::ptr::null_mut(){
            None
         }
         else{
            get_nth_d(prev,nth - 1)
         }
    }
}
#[no_mangle]
pub fn get_nth_db(curr:*mut BetterDoubleLinkedListNode<usize>,nth:usize)->Option<usize>{
    if nth == 0{
        Some(unsafe{(*curr).val})
    }
    else{
         let next = unsafe{(*curr).next};
         if let Some(next) = next{
            get_nth_db(next,nth - 1)
         }
         else{
            None
         }
    }
}
#[no_mangle]
pub fn get_nth_back_db(curr:*mut BetterDoubleLinkedListNode<usize>,nth:usize)->Option<usize>{
    if nth == 0{
        Some(unsafe{(*curr).val})
    }
    else{
         let prev = unsafe{(*curr).prev};
         if let Some(prev) = prev{
            get_nth_db(prev,nth - 1)
         }
         else{
            None
         }
    }
}

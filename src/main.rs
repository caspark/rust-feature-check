fn main() {}

#[no_mangle]
pub fn test_sse2() -> i32 {
    if is_x86_feature_detected!("sse2") {
        1
    } else {
        0
    }
}

#[no_mangle]
pub fn test_sse41() -> i32 {
    if is_x86_feature_detected!("sse4.1") {
        2
    } else {
        0
    }
}


#[no_mangle]
pub fn test_pclmulqdq() -> i32 {
    if is_x86_feature_detected!("pclmulqdq") {
        3
    } else {
        0
    }
}

pub fn is_odd(val: isize) -> bool {
    if val % 2 != 0 {
        true
    } else {
        false
    }
}


#[test]
fn test_odd() {
    assert!(is_odd(1) == true);
    assert!(is_odd(2) == false);    
}
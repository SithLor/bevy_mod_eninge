pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[unsafe(no_mangle)]
pub extern "C" fn add_c(left: u64, right: u64) -> u64 {
    if left == 3 {
        let p: *mut u8 = 0 as *mut u8;
        unsafe {
            *p = 0;
        }
    }
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

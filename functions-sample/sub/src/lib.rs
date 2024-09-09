#[no_mangle]
pub extern "C" fn sub(left: i32, right: i32) -> i32 {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sub(2, 2);
        assert_eq!(result, 0);
    }
}

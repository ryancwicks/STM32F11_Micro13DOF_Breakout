#![no_std]
pub fn hello_world () -> i32 {
    2+2
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

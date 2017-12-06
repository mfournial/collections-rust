pub mod queue;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn test_next(x: i32) -> i32 {
	x + 1
}
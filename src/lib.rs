pub mod queue;
pub mod tree;

pub fn test_next(x: i32) -> i32 {
	x + 1
}

#[cfg(test)]
mod tests {
	use super::*;
	
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn same_as_it() {
		assert_eq!(2, test_next(1));
    }
}

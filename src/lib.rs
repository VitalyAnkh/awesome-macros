
use std::cmp::max;
use std::cmp::min;
#[macro_export]
macro_rules! max {
     ($e: expr) => { $e };
     ($e: expr, $($rest: tt)*) => { max($e, max!($($rest)*)) }
}

#[macro_export]
macro_rules! min {
     ($e: expr) => { $e };
     ($e: expr, $($rest: tt)*) => { min($e, min!($($rest)*)) }
}


#[cfg(test)]
mod tests {
    pub use super::*;
    #[test]
    fn it_works() {
        assert_eq!(max!(1,2,3,4), 4);
    }

    #[test]
    fn char_works(){
        assert_eq!(max!('a','b','c','d'),'d' );
    }

    #[test]
    fn number_char_works(){
        assert_eq!(max!('1','2','3','4'), '4');
    }
}
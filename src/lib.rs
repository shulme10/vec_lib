mod my_vec;

pub use my_vec::MyVec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v1 = MyVec::<u8>::new();
        assert_eq!(v1.size(), 0);
        assert_eq!(v1.capacity(), 0);

        for i in 0..230 {
            v1.push(i);
        }
        // capacity is doubled when passed.
        assert_eq!(v1.size(), 230);
        assert_eq!(v1.capacity(), 256);

        // capacity is already bigger so it shouldn't do anything.
        v1.reserve(10);
        assert_eq!(v1.capacity(), 256);

        v1.reserve(400); // set new capacity
        assert_eq!(v1.capacity(), 400);

        for i in 0..230 {
            v1.push(i);
        }
        assert_eq!(v1.size(), 460);
        assert_eq!(v1.capacity(), 800); // doubled
    }
}

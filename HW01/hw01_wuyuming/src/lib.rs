pub mod problem1;
pub mod problem2;
pub mod problem3;
pub mod problem4;
pub mod tests_provided;

#[cfg(test)]
mod tests {
    use tests_provided::test_sum_small;

    #[test]
    pub fn it_works() {
//        assert_eq!(2 + 2, 4);
        test_sum_small();
    }
}

/// # example doc test
/// ```
/// use rtypes::add;
/// assert_eq!(add(1,2),3);
/// ```

pub fn add(lhs: u32, rhs: u32) -> u32 {
    lhs + rhs
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}

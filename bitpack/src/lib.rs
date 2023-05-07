pub mod bitpack;

#[cfg(test)]
mod tests {
    use crate::bitpack::*;
    #[test]
    fn exploration() {
        assert_eq!(fitss(-4,3), true);
        // assert_eq!(fitss(3,3), true);
    }

}
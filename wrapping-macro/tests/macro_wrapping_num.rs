// cargo expand -p wrapping-macro --verbose --test macro_wrapping_num

//
wrapping_macro::wrapping_int! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct U32Wrapper(pub u32);
}

wrapping_macro::wrapping_float! {
    #[derive(Debug, Clone, PartialEq)]
    pub struct F32Wrapper(pub f32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_methods_and_impl() {
        // Display and FromStr
        let w = "1".parse::<U32Wrapper>().unwrap();
        assert_eq!(w.0, 1);
        assert_eq!(format!("{w}"), "1");

        let w = "1.1".parse::<F32Wrapper>().unwrap();
        assert_eq!(w.0, 1.1);
        assert_eq!(format!("{w}"), "1.1");
    }
}

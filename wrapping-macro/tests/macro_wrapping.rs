// cargo expand -p wrapping-macro --verbose --test macro_wrapping

//
wrapping_macro::wrapping! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct U64Wrapper(u64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_methods_and_impl() {
        let w = U64Wrapper::from_inner(1);
        assert_eq!(w.inner(), &1);
        assert_eq!(w.into_inner(), 1);

        let w = U64Wrapper::from(1);
        let _ = w.abs_diff(1);
    }
}

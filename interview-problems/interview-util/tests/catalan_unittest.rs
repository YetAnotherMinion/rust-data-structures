extern crate yam_interview_util;

#[cfg(test)]
mod tests {
    use yam_interview_util::numbers::catalan;
    #[test]
    fn spot_check_catalan() {
        assert_eq!(1, catalan(1));
        assert_eq!(2, catalan(2));
        assert_eq!(5, catalan(3));
        assert_eq!(14, catalan(4));
        assert_eq!(42, catalan(5));
    }
}


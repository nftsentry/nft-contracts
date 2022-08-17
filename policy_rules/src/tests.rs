#[cfg(test)]
mod tests {
    use crate::policy::init_policies;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_init_policies() {
        let policies = init_policies();
        assert_eq!(policies.version, "0.0.1");
        assert_eq!(policies.limitations.len(), 3);
        assert_eq!(policies.policies.len(), 4);
    }
}
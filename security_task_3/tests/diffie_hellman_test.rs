use security_task_3::diffie_hellman::DiffieHellman;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diffie_hellman1() {
        let algorithm = DiffieHellman;
        let key = algorithm.get_keys(19, 2, 6, 13);
        assert_eq!(7, key[0]);
        assert_eq!(7, key[1]);
    }

    #[test]
    fn test_diffie_hellman2() {
        let algorithm = DiffieHellman;
        let key = algorithm.get_keys(353, 2, 97, 233);
        assert_eq!(81, key[0]);
        assert_eq!(81, key[1]);
    }

    #[test]
    fn test_diffie_hellman3() {
        let algorithm = DiffieHellman;
        let key = algorithm.get_keys(353, 3, 97, 233);
        assert_eq!(160, key[0]);
        assert_eq!(160, key[1]);
    }

    #[test]
    fn test_diffie_hellman_new() {
        let algorithm = DiffieHellman;
        let key = algorithm.get_keys(541, 10, 50, 100);
        assert_eq!(449, key[0]);
        assert_eq!(449, key[1]);
    }
}
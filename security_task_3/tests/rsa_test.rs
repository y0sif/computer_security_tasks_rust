use security_task_3::rsa::RSA;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enc1() {
        let algorithm = RSA;
        let cipher = algorithm.encrypt(11, 17, 88, 7);
        assert_eq!(11, cipher);
    }

    #[test]
    fn test_dec1() {
        let algorithm = RSA;
        let plain = algorithm.decrypt(11, 17, 11, 7);
        assert_eq!(88, plain);
    }

    #[test]
    fn test_enc2() {
        let algorithm = RSA;
        let cipher = algorithm.encrypt(13, 19, 65, 5);
        assert_eq!(221, cipher);
    }

    #[test]
    fn test_dec2() {
        let algorithm = RSA;
        let plain = algorithm.decrypt(13, 19, 221, 5);
        assert_eq!(65, plain);
    }

    #[test]
    fn test_enc3() {
        let algorithm = RSA;
        let cipher = algorithm.encrypt(61, 53, 70, 7);
        assert_eq!(2338, cipher);
    }

    #[test]
    fn test_dec3() {
        let algorithm = RSA;
        let plain = algorithm.decrypt(61, 53, 2338, 7);
        assert_eq!(70, plain);
    }

    #[test]
    fn test_new_enc() {
        let algorithm = RSA;
        let cipher = algorithm.encrypt(257, 337, 18537, 17);
        assert_eq!(12448, cipher);
    }

    #[test]
    fn test_new_dec4() {
        let algorithm = RSA;
        let plain = algorithm.decrypt(257, 337, 12448, 17);
        assert_eq!(18537, plain);
    }
}
use security_task_3::elgamal::ElGamal;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_el_gamal_enc1() {
        let algorithm = ElGamal;
        let cipher = algorithm.encrypt(7187, 4842, 4464, 19, 19); // privateKey = 191
        assert_eq!(2781, cipher[0]);
        assert_eq!(437, cipher[1]);
    }

    #[test]
    fn test_el_gamal_enc2() {
        let algorithm = ElGamal;
        let cipher = algorithm.encrypt(6323, 4736, 2231, 58, 111); // privateKey = 118
        assert_eq!(6066, cipher[0]);
        assert_eq!(899, cipher[1]);
    }

    #[test]
    fn test_el_gamal_dec1() {
        let algorithm = ElGamal;
        let plain = algorithm.decrypt(2781, 437, 191, 7187);
        assert_eq!(19, plain);
    }

    #[test]
    fn test_el_gamal_dec2() {
        let algorithm = ElGamal;
        let plain = algorithm.decrypt(6066, 899, 118, 6323);
        assert_eq!(111, plain);
    }
}
use totp_rs::Algorithm;

#[derive(Debug)]
pub struct Token {
    pub issuer: String,
    pub account_name: String,
    pub secret: String,
}

impl Token {
    pub fn new(issuer: String, account_name: String, secret: String) -> Self {
        Self {
            issuer,
            account_name,
            secret,
        }
    }
}

#[derive(Debug)]
pub struct Otp {
    pub algorithm: Algorithm,
    pub digits: usize,
    pub step: u64,
}

impl Otp {
    pub fn new(algorithm: Algorithm, digits: usize, step: u64) -> Self {
        Self {
            algorithm,
            digits,
            step,
        }
    }
}

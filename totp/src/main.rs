use std::env;
use totp::{Token, Otp};
use totp_rs::{Algorithm, TOTP};
use dotenv::dotenv;

fn env_as_token() -> Token {
    let _ = dotenv().ok();
    let issuer = env::var("ISSUER").expect("Couldn't find the env variable.");
    let account_name = env::var("ACCOUNT_NAME").expect("Couldn't find the env variable.");
    let secret = env::var("SECRET").expect("Couldn't find the env variable");

    Token::new(issuer, account_name, secret)
}

fn auth(token: Token, otp: Otp) -> String {
    format!(
        "otpauth://totp/{}:{}@?secret={}&issuer={}&algorithm={}&digits={}&period={}", 
        token.issuer, 
        token.account_name, 
        token.secret, 
        token.issuer,
        otp.algorithm,
        otp.digits,
        otp.step
    )
}

fn main() {
    let token = env_as_token();
    let otp = Otp::new(Algorithm::SHA1, 6, 30);
    let otpauth = auth(token, otp);
    let totp = TOTP::from_url(otpauth).unwrap();
    
    println!("Your {} token is: {}", totp.issuer.as_ref().unwrap(), totp.generate_current().unwrap_or_default());
}

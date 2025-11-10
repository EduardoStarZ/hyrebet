use crate::env::get_hash;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation, decode};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String
}

#[derive(Serialize)]
pub enum LoginToken {
    Value(String),
    None
}


#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub : String,
    pub exp : usize
}

pub trait ClaimChecker {
    fn check_exp(&self) -> bool;
}

impl ClaimChecker for Claims {
    fn check_exp(&self) -> bool {
        if self.exp - (chrono::Utc::now().timestamp() as usize) == 0 {
            return false; 
        }
        return true;
    }
}

pub fn create_token(info: LoginInfo) -> LoginToken {

    let hash = match get_hash() {
        Some(value) => value,
        None => panic!("Value for hash not found during execution!")
    };
    
 let claims : Claims = Claims { sub : info.username, exp: (chrono::Utc::now() + chrono::Duration::days(10)).timestamp() as usize};
        let mut token : String = match encode(&Header::default(), &claims, &EncodingKey::from_secret(hash.as_ref())) {                               
            
            Ok(token) => token,
            Err(err) => { 
                println!("Error generating token: {}", err);
                return LoginToken::None;
            }
        };

        token.insert_str(0, "Bearer ");

        return LoginToken::Value(token);
}

pub fn check_token_val(tokenized_info : &LoginToken) -> bool {
     let hash = match get_hash() {
        Some(value) => value,
        None => panic!("Value for hash not found during execution!")
    };


    let token : String = match tokenized_info {
        LoginToken::Value(value) => value.to_string(),
        LoginToken::None => return false
    };

    if token.starts_with("Bearer ") {
        let headless : String = token.trim_start_matches("Bearer ").trim().to_string();

        return match decode::<Claims>(&headless, &DecodingKey::from_secret(hash.as_ref()), &Validation::default()) {
            Ok(values) => values.claims.check_exp(),
            Err(_error) => {
                return false
            }
        }

    }
    return false;
}

pub fn get_user_from_token(tokenized_info : &LoginToken) -> Option<String> {
    let hash = match get_hash() {
        Some(value) => value,
        None => panic!("Value for hash not found during execution!")
    };


    let token : String = match tokenized_info {
        LoginToken::Value(value) => value.to_string(),
        LoginToken::None => return None
    };

    if token.starts_with("Bearer ") {
        let headless : String = token.trim_start_matches("Bearer ").trim().to_string();

        return match decode::<Claims>(&headless, &DecodingKey::from_secret(hash.as_ref()), &Validation::default()) {
            Ok(values) => Some(values.claims.sub),
            Err(_) => {
                return None
            }
        }

    }
    return None;

}

use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation, decode};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String
}

#[derive(Serialize)]
pub struct LoginToken(pub Option<String>);


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

pub fn create_token(info: LoginInfo, hash: &str ) -> LoginToken {
    
 let claims : Claims = Claims { sub : info.username, exp: (chrono::Utc::now() + chrono::Duration::hours(3)).timestamp() as usize};
        let mut token : String = match encode(&Header::default(), &claims, &EncodingKey::from_secret(hash.as_ref())) {                               
            
            Ok(token) => token,
            Err(err) => { 
                println!("Error generating token: {}", err);
                return LoginToken(None);
            }
        };

        token.insert_str(0, "Bearer ");

        return LoginToken(Some(token));
}

pub fn check_token_val(tokenized_info : &LoginToken, hash : &str) -> bool {
    let token : String = match &tokenized_info.0 {
        Some(value) => value.to_string(),
        None => return false
    };

    if token.starts_with("Bearer ") {
        let headless : String = token.trim_start_matches("Bearer ").trim().to_string();

        return match decode::<Claims>(&headless, &DecodingKey::from_secret(hash.as_ref()), &Validation::default()) {
            Ok(values) => values.claims.check_exp(),
            Err(error) => {
                println!("{}", error);
                return false
            }
        }

    }
    return false;
}

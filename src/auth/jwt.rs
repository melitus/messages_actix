use chrono::{Duration, Utc};
use jsonwebtoken::{errors::Error,decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::api::v1::user;

use super::claims::{Claims, TokenDetails};
use uuid::Uuid;

const JWT_SECRET: &[u8] = b"JWT_TOKEN_SECRET";
static ONE_HOUR: i64 = 60 * 60; // in seconds
const AES_KEY: &str = "e3Ui2PBkyFl5vUaO";
const ISSUER: &'static str = "http://librevpn.net";

pub fn generate_token(user_id: &Uuid, email: &String) -> Result<TokenDetails, Error> {
    let create_time = Utc::now();
    let expire_time = Utc::now() + Duration::hours(ONE_HOUR);
    // let exp = (now + chrono::Duration::hours(ONE_HOUR)).timestamp();

    let jti = uuid::Uuid::new_v4();
    let mut scopes = vec![String::from("ROLE_MEMBER")];
    let user_type = 10;
    if user_type == 10 {
        scopes.push(String::from("ROLE_ADMIN"));
    }

    let claims = Claims {
        sub: *user_id,
        jti: jti.to_string(),
        email: email.clone(),       
        iat: create_time.timestamp(),
        nbf: create_time.timestamp(),
        exp: expire_time.timestamp(),
        iss: ISSUER.to_string(),
        scopes: scopes.clone()
    };

    let header = Header::new(Algorithm::HS512);
    let token = encode(&header,&claims,&EncodingKey::from_secret(JWT_SECRET))?;
    let token_details = TokenDetails {
        user_id: user_id.to_string(),
        token_uuid: jti,
        expires_in: expire_time.timestamp(),
        token,
        scopes,
    };

    Ok(token_details)
}

// this is decode token
pub fn validate_token(token: &str) -> Result<TokenDetails, Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false;
    let decoded = decode::<Claims>(&token, &DecodingKey::from_secret(JWT_SECRET), &validation)?;
    let user_id =decoded.claims.sub;

    // let user_id = aes::decrypt(&decoded.claims.sub, AES_KEY);
    let token_uuid = Uuid::parse_str(decoded.claims.jti.as_str()).unwrap();
    // let user_id = Uuid::parse_str(decoded.claims.jti.as_str());

    let token_expiry = decoded.claims.exp;
    let scopes = decoded.claims.scopes;

    Ok(TokenDetails {
        token: token.to_string(),
        token_uuid,
        user_id: user_id.to_string(),
        expires_in: token_expiry,
        scopes,
    })
}

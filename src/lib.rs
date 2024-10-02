mod claims;

use jsonwebtoken::{encode, Header, EncodingKey, Algorithm, errors::Result};
use claims::Claims;

pub fn create_jwt(subject: &str, name: &str, secret: &str, algorithm: Algorithm) -> Result<String> {
    let my_claims = Claims::new(subject, name);
    
    let header = Header::new(algorithm);
    let token = encode(&header, &my_claims, &EncodingKey::from_secret(secret.as_ref()))?;
    
    Ok(token)
}

pub fn decode_jwt(token: &str, secret: &str, algorithm: Algorithm) -> Result<jsonwebtoken::TokenData<Claims>> {
    let validation = jsonwebtoken::Validation::new(algorithm);
    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;
    Ok(token_data)
}

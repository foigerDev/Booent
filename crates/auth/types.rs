use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    // user id
    pub sub: String,
    // expiry (unix)      
    pub exp: u64,        
    // issued at  
    pub iat: u64,  
    // not before         
    pub nbf: u64,           
     // issuer (booent-auth)
    pub iss: String,   
    // audience (booent-user)    
    pub aud: String,        
    // token id (for revocation)
    pub jti: String,
}
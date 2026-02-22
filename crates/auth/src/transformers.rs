// use crate::domain_models::GoogleUserClaims;

// impl From<&google_jwt_verify::TokenPayload> for GoogleUserClaims {
//     fn from(payload: &google_jwt_verify::TokenPayload) -> Self {
//         Self {
//             google_user_id: payload.sub.clone(),
//             email: payload.email.clone().unwrap_or_default(),
//             name: payload.name.clone(),
//             picture: payload.picture.clone(),
//             email_verified: payload.email_verified.unwrap_or(false),
//         }
//     }
// }
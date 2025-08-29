use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::OnceLock;

pub const JWT_ISSUER: &str = "AzatAI";
pub const JWT_AUDIENCE: &str = "project-name";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Staff,
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims<'a> {
    pub sub: String,       // Whom token refers to
    pub role: UserRole,    // Custom claim (e.g. admin)
    pub exp: i64,          // Expiration time (as UTC timestamp)
    pub iat: i64,          // Issuance time (as UTC timestamp)
    pub iss: Cow<'a, str>, // Issuer of token (e.g., AzatAI)
    pub aud: Cow<'a, str>, // Audience of token (e.g. project-name)
}

impl<'a> Claims<'a> {
    pub fn new(sub: impl Into<String>, role: UserRole, now: i64, ttl_secs: i64) -> Self {
        Self {
            sub: sub.into(),
            role,
            iat: now,
            exp: now + ttl_secs,
            iss: Cow::Borrowed(JWT_ISSUER),
            aud: Cow::Borrowed(JWT_AUDIENCE),
        }
    }
}

pub struct JwtService {
    enc_key: EncodingKey,
    dec_key: DecodingKey,
    alg: Algorithm,
    validation: Validation,
}

impl JwtService {
    pub fn new(secret: impl AsRef<[u8]>) -> Self {
        let alg = Algorithm::HS256;
        let mut validation = Validation::new(alg);
        validation.validate_exp = true;
        validation.set_issuer(&[JWT_ISSUER]);
        validation.set_audience(&[JWT_AUDIENCE]);
        validation.leeway = 30; // seconds
        Self {
            enc_key: EncodingKey::from_secret(secret.as_ref()),
            dec_key: DecodingKey::from_secret(secret.as_ref()),
            alg,
            validation,
        }
    }

    /// Encode (sign) a token from claims
    pub fn encode<'a>(&self, claims: &Claims<'a>) -> Result<String, jsonwebtoken::errors::Error> {
        let header = jsonwebtoken::Header::new(self.alg);
        jsonwebtoken::encode(&header, claims, &self.enc_key)
    }

    /// Decode + verify. Returns claims if valid.
    pub fn decode(&self, token: &str) -> Result<Claims<'static>, jsonwebtoken::errors::Error> {
        let data = jsonwebtoken::decode::<Claims<'_>>(token, &self.dec_key, &self.validation)?;
        // Ensure we return owned data (no lifetimes tied to input)
        let c = data.claims;
        Ok(Claims {
            sub: c.sub,
            role: c.role,
            exp: c.exp,
            iat: c.iat,
            iss: Cow::Owned(c.iss.into_owned()),
            aud: Cow::Owned(c.aud.into_owned()),
        })
    }
}

// Global, lazily initialized
static JWT_SERVICE: OnceLock<JwtService> = OnceLock::new();

/// Initialize once at startup. Later calls do nothing (first wins cached).
pub fn init(secret: impl AsRef<[u8]>) -> &'static JwtService {
    let key = secret.as_ref().to_vec();
    JWT_SERVICE.get_or_init(move || JwtService::new(key))
}

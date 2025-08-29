mod config;
mod jwt;

use crate::jwt::{Claims, UserRole};
use chrono::Utc;
use ulid::Ulid;

fn main() {
    // Get the config (global static)
    let cfg = config::get();
    println!("Loaded config: {:?}", cfg);
    // Crate a sample Claim
    let ulid = Ulid::new();
    let now = Utc::now().timestamp();
    let claims = Claims::new(
        ulid,
        UserRole::Admin,
        now,              // 2023-01-01 00:00:00 UTC
        cfg.jwt_ttl_secs, // 1 hour
    );

    // Init the JWT service
    let jwt = jwt::init(&cfg.jwt_secret);
    let token = jwt.encode(&claims).expect("Failed to encode token");
    println!("Token: {}", token);

    // Decode the token back to claims
    let claims = jwt.decode(&token).expect("Failed to decode token");
    println!("Claims: {:?}", claims);
}

---
name: "jsonwebtoken"
version: "3"
github: "https://github.com/Keats/jsonwebtoken"
crate: "https://crates.io/crates/jsonwebtoken"
doc: "https://docs.rs/jsonwebtoken/9.3.1/jsonwebtoken/"
---

# `jsonwebtoken`

```toml
jsonwebtoken = { version = "9.3.1", default-features = false }
serde = { version = "1.0.219", features = ["derive"] }
```

JWT has 3 parts:

1. Header
    - Metadata about the token( `alg`, `typ`, etc.)
2. Payload
    - This is the body of the JWT.
    - Consists of claims (registered claims like `exp`, `sub`, `iat`, and custom claims)
3. Signature
    - Used to verify that the sender of the JWT is who it says it is and to ensure that the message wasn't changed along
      the way.
    - Cryptographic proof the payload wasn't tampered with.

So, technically:
- Payload = the whole JSON body
- Claims = the individual key–value pairs inside the payload

## Algorithms

- HS256 (*)
- HS384
- HS512
- RS256
- RS384
- RS512
- PS256
- PS384
- PS512
- ES256
- ES384
- EdDSA





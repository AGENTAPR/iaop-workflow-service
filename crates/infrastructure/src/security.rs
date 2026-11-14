//! Security utilities: token extraction helpers and public-path policy.
//!
//! JWT/JWKS validation is centralized in `iaop-security` (iaop-rust-common);
//! services adopt it once tagged releases exist. This module only provides
//! transport-level helpers that contain no cryptographic logic.

/// Extracts the bearer token from an `Authorization` header value.
/// Returns `None` for missing, malformed, or non-bearer headers.
/// Never logs the token.
pub fn bearer_token(authorization: Option<&str>) -> Option<&str> {
    let value = authorization?;
    let (scheme, token) = value.split_once(' ')?;
    if scheme.eq_ignore_ascii_case("bearer") {
        let token = token.trim();
        if token.is_empty() { None } else { Some(token) }
    } else {
        None
    }
}

/// Paths that bypass authentication. Health and API-docs endpoints are the
/// only permitted public paths; everything else requires a valid token.
pub fn is_public_path(path: &str) -> bool {
    matches!(
        path,
        "/health/live" | "/health/ready" | "/api/docs/openapi.json"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_bearer_token() {
        assert_eq!(
            bearer_token(Some("Bearer abc.def.ghi")),
            Some("abc.def.ghi")
        );
        assert_eq!(bearer_token(Some("bearer  tok ")), Some("tok"));
    }

    #[test]
    fn rejects_missing_or_malformed() {
        assert_eq!(bearer_token(None), None);
        assert_eq!(bearer_token(Some("Basic abc")), None);
        assert_eq!(bearer_token(Some("Bearer ")), None);
    }

    #[test]
    fn public_paths_are_limited() {
        assert!(is_public_path("/health/live"));
        assert!(!is_public_path("/api/v1/agents"));
        assert!(!is_public_path("/health/ready/extra"));
    }
}

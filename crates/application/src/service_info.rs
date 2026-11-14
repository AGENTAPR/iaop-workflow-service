use serde::{Deserialize, Serialize};

/// Query: return static service metadata for `GET /api/v1/system/info`.
#[derive(Debug, Clone)]
pub struct GetServiceInfoQuery {
    pub service_name: String,
    pub service_version: String,
    pub environment: String,
}

/// Response DTO produced by the use case (transport-agnostic).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub service: String,
    pub version: String,
    pub environment: String,
    pub status: &'static str,
}

impl GetServiceInfoQuery {
    pub fn execute(self) -> ServiceInfo {
        ServiceInfo {
            service: self.service_name,
            version: self.service_version,
            environment: self.environment,
            status: "ok",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_service_metadata() {
        let info = GetServiceInfoQuery {
            service_name: "svc".into(),
            service_version: "0.1.0".into(),
            environment: "local".into(),
        }
        .execute();
        assert_eq!(info.service, "svc");
        assert_eq!(info.status, "ok");
    }
}

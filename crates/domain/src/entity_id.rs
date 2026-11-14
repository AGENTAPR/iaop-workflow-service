use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Stable identifier for domain entities and aggregates.
///
/// Wraps a UUIDv7 so identifiers are time-ordered and globally unique
/// without exposing the raw UUID type throughout the domain model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EntityId(Uuid);

impl EntityId {
    /// Generates a new time-ordered (UUIDv7) identifier.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for EntityId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s).map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_id_roundtrips_through_string() {
        let id = EntityId::new();
        let parsed: EntityId = id.to_string().parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn entity_id_is_unique() {
        assert_ne!(EntityId::new(), EntityId::new());
    }
}

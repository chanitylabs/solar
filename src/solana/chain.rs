#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Hash, Copy)]
#[cfg_attr(feature = "axum", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum Chain {
    Solana,
}

impl TryFrom<String> for Chain {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&format!(r#""{}""#, value))
    }
}

impl std::fmt::Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self)
                .expect("failed to serialize chain")
                .trim_matches('"')
        )
    }
}

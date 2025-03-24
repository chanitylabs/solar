#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Hash, Copy)]
#[cfg_attr(feature = "axum", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum Dex {
    RaydiumAmm,
    Pumpfun,
}

impl Dex {
    pub fn to_label(&self) -> String {
        match self {
            Dex::RaydiumAmm => "Raydium AMM".to_string(),
            Dex::Pumpfun => "Pumpfun".to_string(),
        }
    }
}

impl TryFrom<String> for Dex {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&format!(r#""{}""#, value))
    }
}

impl std::fmt::Display for Dex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self)
                .expect("failed to serialize dex")
                .trim_matches('"')
        )
    }
}

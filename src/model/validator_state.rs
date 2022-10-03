use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ValidatorState {
    pub height: String,
    pub round: i64,
    pub step: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signbytes: Option<String>,
}

impl Default for ValidatorState {
    fn default() -> Self {
        Self {
            height: String::from("0"),
            round: 0,
            step: 0,
            signature: None,
            signbytes: None,
        }
    }
}

impl ValidatorState {
    pub(crate) fn into_model(self) -> Self {
        self
    }
}

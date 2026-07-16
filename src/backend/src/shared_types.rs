use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../shared/types/")]
pub struct ServerStatus {
    #[ts(type = "number")]
    pub timestamp: u64,
    pub value: f64,
    pub label: String,
}

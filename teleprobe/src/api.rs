use probe_rs::probe::DebugProbeSelector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub name: String,
    pub chip: String,
    #[serde(serialize_with = "string_serialize")]
    pub probe: DebugProbeSelector,
    pub connect_under_reset: bool,
    pub speed: Option<u32>,
    pub up: bool,
    pub power_reset: bool,
    pub cycle_delay_seconds: f64,
    pub max_settle_time_millis: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetList {
    pub targets: Vec<Target>,
}

fn string_serialize<S>(value: &DebugProbeSelector, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    value.to_string().serialize(serializer)
}

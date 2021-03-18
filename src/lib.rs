use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Measurement {
    pub created_at: NaiveDateTime,
    pub vehicle_id: Uuid,
    pub engine_temperature: f32,
    pub engine_power: f32,
    pub accelerometer: Vec<f32>,
}

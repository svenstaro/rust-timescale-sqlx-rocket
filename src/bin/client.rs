use chrono::prelude::*;
use uuid::Uuid;

use rust_timescale_sqlx_rocket::Measurement;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let m = Measurement {
        created_at: NaiveDateTime::from_timestamp(10, 0),
        vehicle_id: Uuid::new_v4(),
        engine_temperature: 0.,
        engine_power: 0.,
        accelerometer: vec![1337., 1337., 1337.],
    };

    let resp = client
        .post("http://localhost:8000/measure")
        .json(&m)
        .send()
        .await?;
    dbg!(&resp);
    let resp_body = resp.text().await?;
    dbg!(&resp_body);
    let resp_m: Measurement = serde_json::from_str(&resp_body)?;
    println!("{:#?}", resp_m);
    Ok(())
}

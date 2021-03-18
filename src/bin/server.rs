use chrono::prelude::*;
use dotenv_codegen::dotenv;
use rocket::State;
use rocket_contrib::json::Json;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use uuid::Uuid;

use rust_timescale_sqlx_rocket::Measurement;

async fn create_db_pool() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(dotenv!("DATABASE_URL"))
        .await?;
    Ok(pool)
}

#[rocket::post("/measure", format = "json", data = "<measurement>")]
async fn insert_measurement(
    measurement: Json<Measurement>,
    pool: State<'_, sqlx::Pool<sqlx::Postgres>>,
) -> Result<Json<Measurement>, String> {
    println!("lolol");
    let rec = sqlx::query_as!(Measurement,
        r#"INSERT INTO vehicle_measurements (created_at, vehicle_id, engine_temperature, engine_power, accelerometer)
        VALUES ( $1, $2, $3, $4, $5 )"#,
        measurement.created_at, measurement.vehicle_id, measurement.engine_temperature, measurement.engine_power, &measurement.accelerometer)
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    println!("rofl");
    // println!("hi {}", rec.len());

    let m = Measurement {
        created_at: NaiveDateTime::from_timestamp(10, 0),
        vehicle_id: Uuid::new_v4(),
        engine_temperature: 0.,
        engine_power: 0.,
        accelerometer: vec![0.1, 0.2, 0.3],
    };
    Ok(Json(m))
}

#[rocket::get("/analyze")]
async fn analyze(pool: State<'_, sqlx::Pool<sqlx::Postgres>>) -> Result<String, String> {
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&*pool)
        .await
        .map_err(|_| "lol".to_string())?;

    Ok(row.0.to_string())
}

#[rocket::launch]
async fn rocket() -> rocket::Rocket {
    let pool = create_db_pool().await.unwrap();
    rocket::ignite()
        .manage(pool)
        .mount("/", rocket::routes![insert_measurement, analyze])
        .register(rocket::catchers![not_found])
}

#[rocket::catch(404)]
fn not_found(_req: &'_ rocket::Request<'_>) -> &'static str {
    "404 Not Found"
}

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;
CREATE TABLE "vehicle_measurements" (
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    vehicle_id UUID NOT NULL DEFAULT uuid_generate_v4(),
    engine_temperature REAL NOT NULL,
    engine_power REAL NOT NULL,
    accelerometer REAL[3] NOT NULL
);
SELECT create_hypertable('vehicle_measurements', 'created_at');

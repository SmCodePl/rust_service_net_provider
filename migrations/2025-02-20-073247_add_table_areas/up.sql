-- Your SQL goes here
CREATE TABLE areas (
    id SERIAL PRIMARY KEY,     
    country_code TEXT NOT NULL, 
    zip_code TEXT NOT NULL,
    place_name TEXT NOT NULL,
    admin_name1 TEXT NOT NULL,
    admin_code1 TEXT NOT NULL,
    admin_name2 TEXT NOT NULL,
    admin_code2 TEXT NOT NULL,
    latitude FLOAT NOT NULL DEFAULT 0.0,
    longitude FLOAT NOT NULL DEFAULT 0.0,
    accuracy FLOAT NOT NULL DEFAULT 0.0,
    created_at TIMESTAMP(6) WITH TIME ZONE  DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(6) WITH TIME ZONE  DEFAULT CURRENT_TIMESTAMP
)
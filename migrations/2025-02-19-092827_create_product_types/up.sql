-- Your SQL goes here
CREATE TABLE product_types (
    id SERIAL PRIMARY KEY,
    name text NOT NULL,
    description text not NULL,
    created_at TIMESTAMP(6) WITH TIME ZONE , 
    updated_at TIMESTAMP(6) WITH TIME ZONE ,
    timestamp TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT (now() at time zone 'utc')
)

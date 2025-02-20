CREATE TABLE product_areas (
    id SERIAL PRIMARY KEY,
    product_id INT ,
    area_id INT,
    timestamp TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT (now() at time zone 'utc'), 
    constraint fk_products foreign key (product_id) references products(id), 
    constraint fk_areas foreign key (area_id) references areas(id)
  
);
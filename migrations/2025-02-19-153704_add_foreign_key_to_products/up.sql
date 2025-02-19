ALTER TABLE products
ADD CONSTRAINT fk_product_type
FOREIGN KEY (product_type_id)
REFERENCES product_types(id);

DO $$
DECLARE
    type_id INTEGER;
BEGIN
    SELECT id INTO type_id
    FROM product_types
    WHERE name = 'Short Range'
    LIMIT 1;

	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)  
	VALUES('Internet 100M', 'Internet 100M upload 100M download 100M', 2800, 20, 100, 345, false, false, false, true,type_id,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
                            
	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)
	VALUES('Internet 200M','Internet 200M download 200M upload 200M',2800,30,200,390, false, false, false, true,type_id ,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
	
	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)
	VALUES('Internet 400M','Internet 400M download 400M upload 200M',2800,40,400,450, false, false, false, true,type_id ,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
	
	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)
	VALUES('Internet 600M','Internet 600M download 600M upload 300M',2800,45,585,600, False, false, false, true,type_id,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
	
	                                                   
	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)
	VALUES( 'Internet 1g','Internet 1G download 1g upload 600M',2900,45,585,660, False, false, false, true,type_id ,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
END $$;
                                                     



DO $$
DECLARE
    type_id INTEGER;
BEGIN

  SELECT id INTO type_id
    FROM product_types
    WHERE name = 'Backbone'
    LIMIT 1;

	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)  
	VALUES('Internet 100M', 'Internet 100M upload 100M download 100M', 2800, 20, 100, 428, false, false, false, true,type_id,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
                            
	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)
	VALUES('Internet 200M','Internet 200M download 200M upload 200M',2800,30,200,474, false, false, false, true,type_id ,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
	
	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)
	VALUES('Internet 400M','Internet 400M download 400M upload 200M',2800,40,400,548, false, false, false, true,type_id ,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
	
	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)
	VALUES('Internet 600M','Internet 600M download 600M upload 300M',2800,45,585,600, False, false, false, true,type_id,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
	
	                                                   
	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)
	VALUES( 'Internet 1g','Internet 1G download 1g upload 600M',2900,45,585,660, False, false, false, true,type_id ,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
END $$;
                


DO $$
DECLARE
    type_id INTEGER;
BEGIN

   SELECT id INTO type_id
    FROM product_types
    WHERE name = 'Regio'
    LIMIT 1;

	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)  
	VALUES('Internet 100M', 'Internet 100M upload 100M download 100M', 2800, 20, 100, 585, false, false, false, true,type_id,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
                            
	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)
	VALUES('Internet 200M','Internet 200M download 200M upload 200M',2800,30,200,624, false, false, false, true,type_id ,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
	
	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)
	VALUES('Internet 400M','Internet 400M download 400M upload 200M',2800,40,400,690, false, false, false, true,type_id ,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
	
	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)
	VALUES('Internet 600M','Internet 600M download 600M upload 300M',2800,45,585,735, False, false, false, true,type_id,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
	
	                                                   
	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)
	VALUES( 'Internet 1g','Internet 1G download 1g upload 600M',2900,45,585,803, False, false, false, true,type_id ,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);



END $$;
           
DO $$
DECLARE
    type_id INTEGER;
BEGIN

  
  SELECT id INTO type_id
    FROM product_types
    WHERE name = 'Country'
    LIMIT 1;

	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)  
	VALUES('Internet 100M', 'Internet 100M upload 100M download 100M', 2800, 20, 100, 675, false, false, false, true,type_id,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
                            
	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)
	VALUES('Internet 200M','Internet 200M download 200M upload 200M',2800,30,200,720, false, false, false, true,type_id ,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
	
	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)
	VALUES('Internet 400M','Internet 400M download 400M upload 200M',2800,40,400,780, false, false, false, true,type_id ,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
	
	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)
	VALUES('Internet 600M','Internet 600M download 600M upload 300M',2800,45,585,825, False, false, false, true,type_id,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
	
	                                                   
	INSERT INTO products ("name", description, tdc_otc, price, speed, mrc, is_unlimited, is_discounted, is_promo, is_active, product_type_id, currency, created_at, updated_at)
	VALUES( 'Internet 1g','Internet 1G download 1g upload 600M',2900,45,585,885, False, false, false, true,type_id ,'NA'::character varying, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

 
END $$;
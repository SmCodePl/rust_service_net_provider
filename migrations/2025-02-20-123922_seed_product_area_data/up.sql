DO $$
DECLARE
    f RECORD;
    v_limit INTEGER := 4119;
BEGIN
    FOR f IN 
        SELECT p.name, p.id
        FROM products p
        WHERE p.product_type_id = 13
        ORDER BY id ASC
    LOOP
        INSERT INTO product_areas (product_id, area_id, "timestamp") 
        SELECT f.id, a.id, NOW()
        FROM areas a 
        ORDER BY a.id 
        LIMIT 4119;
    END LOOP;
END;
$$;


DO $$
DECLARE
    f RECORD;
    v_limit INTEGER := 4119;
BEGIN
    FOR f IN 
        SELECT p.name, p.id
        FROM products p
        WHERE p.product_type_id = 13
        ORDER BY id ASC
    LOOP
        INSERT INTO product_areas (product_id, area_id, "timestamp") 
        SELECT f.id, a.id, NOW()
        FROM areas a 
        ORDER BY a.id 
        OFFSET 4119
		LIMIT 4119;
    END LOOP;
END;
$$;

DO $$
DECLARE
    f RECORD;
    v_limit INTEGER := 4119;
BEGIN
    FOR f IN 
        SELECT p.name, p.id
        FROM products p
        WHERE p.product_type_id = 13
        ORDER BY id ASC
    LOOP
        INSERT INTO product_areas (product_id, area_id, "timestamp") 
        SELECT f.id, a.id, NOW()
        FROM areas a 
        ORDER BY a.id 
        OFFSET 8238
		LIMIT 4119;
    END LOOP;
END;
$$;


DO $$
DECLARE
    f RECORD;
    v_limit INTEGER := 4119;
BEGIN
    FOR f IN 
        SELECT p.name, p.id
        FROM products p
        WHERE p.product_type_id = 13
        ORDER BY id ASC
    LOOP
        INSERT INTO product_areas (product_id, area_id, "timestamp") 
        SELECT f.id, a.id, NOW()
        FROM areas a 
        ORDER BY a.id 
        OFFSET 12357
		LIMIT 4119;
    END LOOP;
END;
$$;

DO $$
DECLARE
    f RECORD;
    v_limit INTEGER := 4119;
BEGIN
    FOR f IN 
        SELECT p.name, p.id
        FROM products p
        WHERE p.product_type_id = 13
        ORDER BY id ASC
    LOOP
        INSERT INTO product_areas (product_id, area_id, "timestamp") 
        SELECT f.id, a.id, NOW()
        FROM areas a 
        ORDER BY a.id 
        OFFSET 16476
		LIMIT 4119;
    END LOOP;
END;
$$;
// @generated automatically by Diesel CLI.

diesel::table! {
    areas (id) {
        id -> Int4,
        country_code -> Text,
        zip_code -> Text,
        place_name -> Text,
        admin_name1 -> Text,
        admin_code1 -> Text,
        admin_name2 -> Text,
        admin_code2 -> Text,
        latitude -> Float8,
        longitude -> Float8,
        accuracy -> Float8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    product_areas (id) {
        id -> Int4,
        product_id -> Nullable<Int4>,
        area_id -> Nullable<Int4>,
        timestamp -> Timestamptz,
    }
}

diesel::table! {
    product_types (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        timestamp -> Timestamptz,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 500]
        description -> Varchar,
        tdc_otc -> Int4,
        price -> Float8,
        speed -> Int4,
        mrc -> Int4,
        is_unlimited -> Bool,
        is_discounted -> Bool,
        is_promo -> Bool,
        is_active -> Bool,
        product_type_id -> Int4,
        #[max_length = 4]
        currency -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(product_areas -> areas (area_id));
diesel::joinable!(product_areas -> products (product_id));
diesel::joinable!(products -> product_types (product_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    areas,
    product_areas,
    product_types,
    products,
);

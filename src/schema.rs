// @generated automatically by Diesel CLI.

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

diesel::joinable!(products -> product_types (product_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    product_types,
    products,
);

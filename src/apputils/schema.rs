table! {
    users (id) {
        id -> Int4,
        email -> Text,
        username -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}
table! {
    article (id) {
        id -> Int4,
        user_id -> Int4,
        category -> Text,
        title -> Text,
        body -> Text,
        created_at -> Timestamp,
    }
}
table! {
    cloudenvironments (id) {
        id -> Int4,
        env_name -> Text,
        created_at -> Timestamp,
    }
}
table! {
    products (id) {
        id -> Int4,
        prod_name -> Text,
        created_at -> Timestamp,
    }
}
table! {
    productowner (id) {
        id -> Int4,
        name -> Text,
        created_at -> Timestamp,
    }
}
table! {
    usersenvs (id) {
        id -> Int4,
        user_id -> Int4,
        env_id -> Int4,
        po_id -> Int4,
        start_date -> Timestamp,
        max_duration -> Text,
        lease_period -> Text,
        status -> Int4,
        env_type -> Text,
        created_at -> Timestamp,
    }
}
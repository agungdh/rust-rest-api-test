// @generated automatically by Diesel CLI.

diesel::table! {
    departments (id) {
        id -> Int4,
        uuid -> Text,
        name -> Text,
        created_at -> Text,
        updated_at -> Nullable<Text>,
    }
}

diesel::table! {
    employees (id) {
        id -> Int4,
        uuid -> Text,
        name -> Text,
        email -> Text,
        position -> Text,
        salary -> Int4,
        department_id -> Int4,
        department_uuid -> Text,
        created_at -> Text,
        updated_at -> Nullable<Text>,
    }
}

diesel::joinable!(employees -> departments (department_id));

diesel::allow_tables_to_appear_in_same_query!(departments, employees,);

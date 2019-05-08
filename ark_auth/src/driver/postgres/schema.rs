table! {
    /// Representation of the `auth_csrf` table.
    ///
    /// (Automatically generated by Diesel.)
    auth_csrf (csrf_key) {
        /// The `created_at` column of the `auth_csrf` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `csrf_key` column of the `auth_csrf` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        csrf_key -> Varchar,
        /// The `csrf_value` column of the `auth_csrf` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        csrf_value -> Varchar,
        /// The `service_id` column of the `auth_csrf` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        service_id -> Int8,
    }
}

table! {
    /// Representation of the `auth_key` table.
    ///
    /// (Automatically generated by Diesel.)
    auth_key (key_id) {
        /// The `created_at` column of the `auth_key` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `auth_key` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
        /// The `key_id` column of the `auth_key` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        key_id -> Int8,
        /// The `key_name` column of the `auth_key` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        key_name -> Varchar,
        /// The `key_value` column of the `auth_key` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        key_value -> Varchar,
        /// The `service_id` column of the `auth_key` table.
        ///
        /// Its SQL type is `Nullable<Int8>`.
        ///
        /// (Automatically generated by Diesel.)
        service_id -> Nullable<Int8>,
        /// The `user_id` column of the `auth_key` table.
        ///
        /// Its SQL type is `Nullable<Int8>`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Nullable<Int8>,
    }
}

table! {
    /// Representation of the `auth_service` table.
    ///
    /// (Automatically generated by Diesel.)
    auth_service (service_id) {
        /// The `created_at` column of the `auth_service` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `auth_service` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
        /// The `service_id` column of the `auth_service` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        service_id -> Int8,
        /// The `service_name` column of the `auth_service` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        service_name -> Varchar,
        /// The `service_url` column of the `auth_service` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        service_url -> Varchar,
    }
}

table! {
    /// Representation of the `auth_user` table.
    ///
    /// (Automatically generated by Diesel.)
    auth_user (user_id) {
        /// The `created_at` column of the `auth_user` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `auth_user` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
        /// The `user_id` column of the `auth_user` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int8,
        /// The `user_name` column of the `auth_user` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        user_name -> Varchar,
        /// The `user_email` column of the `auth_user` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        user_email -> Varchar,
        /// The `user_password_hash` column of the `auth_user` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        user_password_hash -> Nullable<Varchar>,
        /// The `user_password_revision` column of the `auth_user` table.
        ///
        /// Its SQL type is `Nullable<Int8>`.
        ///
        /// (Automatically generated by Diesel.)
        user_password_revision -> Nullable<Int8>,
    }
}

joinable!(auth_csrf -> auth_service (service_id));
joinable!(auth_key -> auth_service (service_id));
joinable!(auth_key -> auth_user (user_id));

allow_tables_to_appear_in_same_query!(auth_csrf, auth_key, auth_service, auth_user,);

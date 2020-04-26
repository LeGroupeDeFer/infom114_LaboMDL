table! {
    addresses (id) {
        id -> Unsigned<Integer>,
        street -> Varchar,
        number -> Unsigned<Integer>,
        box_number -> Nullable<Varchar>,
        city -> Varchar,
        zipcode -> Varchar,
        country -> Varchar,
    }
}

table! {
    roles (id) {
        id -> Unsigned<Integer>,
        name -> Nullable<Varchar>,
        color -> Varchar,
    }
}

table! {
    tokens (id) {
        id -> Unsigned<Integer>,
        hash -> Varchar,
        creation_date -> Timestamp,
        expiration_date -> Nullable<Timestamp>,
        count -> Integer,
        consumed -> Bool,
    }
}

table! {
    users (id) {
        id -> Unsigned<Integer>,
        email -> Varchar,
        password -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
        address -> Nullable<Unsigned<Integer>>,
        phone -> Nullable<Varchar>,
        creation_date -> Timestamp,
        last_connection -> Timestamp,
        activation_token -> Nullable<Unsigned<Integer>>,
        recovery_token -> Nullable<Unsigned<Integer>>,
        refresh_token -> Nullable<Unsigned<Integer>>,
        active -> Bool,
    }
}

table! {
    users_roles (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        role_id -> Unsigned<Integer>,
    }
}

joinable!(users -> addresses (address));
joinable!(users_roles -> roles (role_id));
joinable!(users_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    addresses,
    roles,
    tokens,
    users,
    users_roles,
);

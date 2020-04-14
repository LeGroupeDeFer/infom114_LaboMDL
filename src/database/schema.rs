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
    }
}

table! {
    tags (id) {
        id -> Unsigned<Integer>,
        label -> Varchar,
    }
}

table! {
    tags_subscription (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        tag_id -> Unsigned<Integer>,
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
        token -> Nullable<Varchar>,
        active -> Bool,
    }
}

table! {
    users_roles (user, role) {
        user -> Unsigned<Integer>,
        role -> Unsigned<Integer>,
    }
}

joinable!(tags_subscription -> tags (tag_id));
joinable!(tags_subscription -> users (user_id));
joinable!(users -> addresses (address));
joinable!(users_roles -> roles (role));
joinable!(users_roles -> users (user));

allow_tables_to_appear_in_same_query!(
    addresses,
    roles,
    tags,
    tags_subscription,
    users,
    users_roles,
);

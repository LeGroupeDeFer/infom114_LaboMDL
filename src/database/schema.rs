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
    capabilities (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
    }
}

table! {
    roles (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        color -> Varchar,
    }
}

table! {
    roles_capabilities (id) {
        id -> Unsigned<Integer>,
        role_id -> Unsigned<Integer>,
        capability_id -> Unsigned<Integer>,
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
    users_roles (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        role_id -> Unsigned<Integer>,
    }
}

joinable!(roles_capabilities -> capabilities (capability_id));
joinable!(roles_capabilities -> roles (role_id));
joinable!(tags_subscription -> tags (tag_id));
joinable!(tags_subscription -> users (user_id));
joinable!(users -> addresses (address));
joinable!(users_roles -> roles (role_id));
joinable!(users_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    addresses,
    capabilities,
    roles,
    roles_capabilities,
    tags,
    tags_subscription,
    users,
    users_roles,
);

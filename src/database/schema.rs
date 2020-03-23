table! {
    addresses (id) {
        id -> Unsigned<Integer>,
        street -> Varchar,
        number -> Unsigned<Integer>,
        box_number -> Nullable<Varchar>,
        city -> Varchar,
        zipcode -> Unsigned<Integer>,
        country -> Varchar,
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
    roles (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
    }
}

table! {
    users_roles (user, role) {
        user -> Unsigned<Integer>,
        role -> Unsigned<Integer>,
    }
}

joinable!(users -> addresses(id));
joinable!(users_roles -> users(user));
joinable!(users_roles -> roles(role));
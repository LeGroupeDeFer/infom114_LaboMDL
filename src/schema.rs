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
    users (id) {
        id -> Unsigned<Integer>,
        email -> Varchar,
        password -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
        street -> Nullable<Varchar>,
        number -> Nullable<Unsigned<Integer>>,
        city -> Nullable<Varchar>,
        zipcode -> Nullable<Unsigned<Integer>>,
        country -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    addresses,
    roles,
    users,
);

table! {
    users (id) {
        id -> Unsigned<Integer>,
        password -> Varchar,
        email -> Varchar,
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

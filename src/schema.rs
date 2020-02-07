table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        password -> Varchar,
        email -> Nullable<Varchar>,
        firstname -> Nullable<Varchar>,
        lastname -> Nullable<Varchar>,
    }
}

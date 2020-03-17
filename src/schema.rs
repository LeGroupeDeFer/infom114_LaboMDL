table! {
    posts (id) {
        id -> Unsigned<Integer>,
        title -> Nullable<Varchar>,
        content -> Mediumtext,
        authorid -> Unsigned<Integer>,
        created_at -> Nullable<Datetime>,
        modified_at -> Nullable<Datetime>,
        reply_to -> Nullable<Unsigned<Integer>>,
    }
}

table! {
    post_tag_map (post_id, tag_id) {
        post_id -> Unsigned<Integer>,
        tag_id -> Unsigned<Integer>,
    }
}

table! {
    tags (id) {
        id -> Unsigned<Integer>,
        description -> Varchar,
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

table! {
    votes (post_id, vote_authorid) {
        post_id -> Unsigned<Integer>,
        vote_authorid -> Unsigned<Integer>,
        voted_at -> Nullable<Datetime>,
        is_vote_up -> Bool,
    }
}

joinable!(post_tag_map -> posts (post_id));
joinable!(post_tag_map -> tags (tag_id));
joinable!(posts -> users (authorid));
joinable!(votes -> posts (post_id));
joinable!(votes -> users (vote_authorid));

allow_tables_to_appear_in_same_query!(
    posts,
    post_tag_map,
    tags,
    users,
    votes,
);

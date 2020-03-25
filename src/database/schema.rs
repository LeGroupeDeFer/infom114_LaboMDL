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
    comments (id) {
        id -> Unsigned<Integer>,
        post_id -> Unsigned<Integer>,
        content -> Mediumtext,
        authorid -> Unsigned<Integer>,
        created_at -> Nullable<Datetime>,
        modified_at -> Nullable<Datetime>,
        nb_votes -> Unsigned<Integer>,
        parent_id -> Nullable<Unsigned<Integer>>,
    }
}

table! {
    posts (id) {
        id -> Unsigned<Integer>,
        title -> Varchar,
        content -> Mediumtext,
        post_type -> Varchar,
        authorid -> Unsigned<Integer>,
        created_at -> Nullable<Datetime>,
        modified_at -> Nullable<Datetime>,
        nb_votes -> Unsigned<Integer>,
    }
}

table! {
    posts_tags (post_id, tag_id) {
        post_id -> Unsigned<Integer>,
        tag_id -> Unsigned<Integer>,
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

table! {
    votes_comments (comment_id, vote_authorid) {
        comment_id -> Unsigned<Integer>,
        vote_authorid -> Unsigned<Integer>,
        voted_at -> Nullable<Datetime>,
        vote_value -> Bool,
    }
}

table! {
    votes_posts (post_id, vote_authorid) {
        post_id -> Unsigned<Integer>,
        vote_authorid -> Unsigned<Integer>,
        voted_at -> Nullable<Datetime>,
        vote_value -> Bool,
    }
}

joinable!(comments -> posts (post_id));
joinable!(comments -> users (authorid));
joinable!(posts -> users (authorid));
joinable!(posts_tags -> posts (post_id));
joinable!(posts_tags -> tags (tag_id));
joinable!(users -> addresses (address));
joinable!(users_roles -> roles (role));
joinable!(users_roles -> users (user));
joinable!(votes_comments -> comments (comment_id));
joinable!(votes_comments -> users (vote_authorid));
joinable!(votes_posts -> posts (post_id));
joinable!(votes_posts -> users (vote_authorid));

allow_tables_to_appear_in_same_query!(
    addresses,
    comments,
    posts,
    posts_tags,
    roles,
    tags,
    users,
    users_roles,
    votes_comments,
    votes_posts,
);

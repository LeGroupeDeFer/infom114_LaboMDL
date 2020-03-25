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
joinable!(votes_comments -> comments (comment_id));
joinable!(votes_comments -> users (vote_authorid));
joinable!(votes_posts -> posts (post_id));
joinable!(votes_posts -> users (vote_authorid));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    posts_tags,
    tags,
    users,
    votes_comments,
    votes_posts,
);

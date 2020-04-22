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
    comments (id) {
        id -> Unsigned<Integer>,
        post_id -> Unsigned<Integer>,
        content -> Mediumtext,
        authorid -> Unsigned<Integer>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
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
        updated_at -> Nullable<Datetime>,
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
        name -> Varchar,
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
joinable!(posts_tags -> posts (post_id));
joinable!(votes_comments -> comments (comment_id));
joinable!(votes_posts -> posts (post_id));

allow_tables_to_appear_in_same_query!(
    addresses,
    comments,
    posts,
    posts_tags,
    roles,
    votes_comments,
    votes_posts,
);

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
    comments (id) {
        id -> Unsigned<Integer>,
        post_id -> Unsigned<Integer>,
        parent_id -> Nullable<Unsigned<Integer>>,
        content -> Mediumtext,
        author_id -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        hidden_at -> Nullable<Timestamp>,
        locked_at -> Nullable<Timestamp>,
        votes -> Unsigned<Integer>,
        score -> Integer,
    }
}

table! {
    comments_reports (comment_id, user_id) {
        comment_id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        reported_at -> Timestamp,
        reason -> Nullable<Mediumtext>,
    }
}

table! {
    posts (id) {
        id -> Unsigned<Integer>,
        title -> Varchar,
        content -> Mediumtext,
        post_type -> Varchar,
        author_id -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        hidden_at -> Nullable<Timestamp>,
        locked_at -> Nullable<Timestamp>,
        votes -> Unsigned<Bigint>,
        score -> Bigint,
        rank -> Double,
    }
}

table! {
    posts_reports (post_id, user_id) {
        post_id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        reported_at -> Timestamp,
        reason -> Nullable<Mediumtext>,
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
    tokens (id) {
        id -> Unsigned<Integer>,
        hash -> Varchar,
        creation_date -> Timestamp,
        expiration_date -> Nullable<Timestamp>,
        count -> Integer,
        consumed -> Bool,
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
        activation_token -> Nullable<Unsigned<Integer>>,
        recovery_token -> Nullable<Unsigned<Integer>>,
        refresh_token -> Nullable<Unsigned<Integer>>,
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

table! {
    votes_comments (comment_id, user_id) {
        comment_id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        voted_at -> Timestamp,
        vote_value -> Bool,
    }
}

table! {
    votes_posts (post_id, user_id) {
        post_id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        voted_at -> Timestamp,
        vote_value -> Smallint,
    }
}

joinable!(comments -> posts (post_id));
joinable!(comments -> users (author_id));
joinable!(comments_reports -> comments (comment_id));
joinable!(comments_reports -> users (user_id));
joinable!(posts -> users (author_id));
joinable!(posts_reports -> posts (post_id));
joinable!(posts_reports -> users (user_id));
joinable!(posts_tags -> posts (post_id));
joinable!(posts_tags -> tags (tag_id));
joinable!(roles_capabilities -> capabilities (capability_id));
joinable!(roles_capabilities -> roles (role_id));
joinable!(tags_subscription -> tags (tag_id));
joinable!(tags_subscription -> users (user_id));
joinable!(users -> addresses (address));
joinable!(users_roles -> roles (role_id));
joinable!(users_roles -> users (user_id));
joinable!(votes_comments -> comments (comment_id));
joinable!(votes_comments -> users (user_id));
joinable!(votes_posts -> posts (post_id));
joinable!(votes_posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    addresses,
    capabilities,
    comments,
    comments_reports,
    posts,
    posts_reports,
    posts_tags,
    roles,
    roles_capabilities,
    tags,
    tags_subscription,
    tokens,
    users,
    users_roles,
    votes_comments,
    votes_posts,
);

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
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
        hidden_at -> Nullable<Datetime>,
        score -> Integer,
    }
}

table! {
    comments_reports (comment_id, report_id) {
        comment_id -> Unsigned<Integer>,
        report_id -> Unsigned<Integer>,
    }
}

table! {
    posts (id) {
        id -> Unsigned<Integer>,
        title -> Varchar,
        content -> Mediumtext,
        post_type -> Varchar,
        author_id -> Unsigned<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
        deleted_at -> Nullable<Datetime>,
        hidden_at -> Nullable<Datetime>,
        locked_at -> Nullable<Datetime>,
        score -> Integer,
    }
}

table! {
    posts_reports (post_id, report_id) {
        post_id -> Unsigned<Integer>,
        report_id -> Unsigned<Integer>,
    }
}

table! {
    posts_tags (post_id, tag_id) {
        post_id -> Unsigned<Integer>,
        tag_id -> Unsigned<Integer>,
    }
}

table! {
    reports (id) {
        id -> Unsigned<Integer>,
        reason -> Nullable<Mediumtext>,
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
joinable!(comments -> users (author_id));
joinable!(comments_reports -> comments (comment_id));
joinable!(comments_reports -> reports (report_id));
joinable!(posts -> users (author_id));
joinable!(posts_reports -> posts (post_id));
joinable!(posts_reports -> reports (report_id));
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
joinable!(votes_comments -> users (vote_authorid));
joinable!(votes_posts -> posts (post_id));
joinable!(votes_posts -> users (vote_authorid));

allow_tables_to_appear_in_same_query!(
    addresses,
    capabilities,
    comments,
    comments_reports,
    posts,
    posts_reports,
    posts_tags,
    reports,
    roles,
    roles_capabilities,
    tags,
    tags_subscription,
    users,
    users_roles,
    votes_comments,
    votes_posts,
);

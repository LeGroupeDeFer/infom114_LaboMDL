pub use super::schema::{
    addresses::dsl::addresses as addresses_table,
    capabilities::dsl::capabilities as capabilities_table,
    comments::dsl::comments as comments_table,
    posts::dsl::posts as posts_table,
    posts_tags::dsl::posts_tags as posts_tags_table,
    roles::dsl::roles as roles_table,
    roles_capabilities::dsl::roles_capabilities as roles_capabilities_table,
    tags::dsl::tags as tags_table,
    tags_subscription::dsl::tags_subscription as tags_subscription_table,
    users::dsl::users as users_table,
    users_roles::dsl::users_roles as users_roles_table,
    votes_comments::dsl::votes_comments as votes_comments_table,
    votes_posts::dsl::votes_posts as votes_posts_table,
    tokens::dsl::tokens as tokens_table
};

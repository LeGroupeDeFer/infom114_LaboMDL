use crate::database::models::prelude::*;
use crate::lib;
use diesel::MysqlConnection;
use either::Either;

/// This seeder will create a user and some post on the database
///
/// * User :
///     * email     = `alan.smithee@unamur.be`
///     * password  = `author`
/// * Post :
///     * 5 valid posts (without any comments, nor votes)
///     * one deleted post (title = `Deleted post`)
///     * one hidden post (title = `Hidden post`)
///     * one locked post (title = `Locked post`)
///
pub fn seed_test_posts(conn: &MysqlConnection) {
    let author = init_author(&conn);
    init_tags(&conn);
    let x = 5;

    // create x valid post
    for i in 1..=x {
        // let tag = if i % 2 == 0 { "even" } else { "odd" };
        let post_minima = PostMinima {
            author_id: author.id,
            title: format!("Valid post #{}", i),
            content: lib::lorem_ipsum(),
            kind: 0,
        };

        let p = match PostEntity::insert(&conn, &post_minima).unwrap() {
            Either::Left(post) => post,
            Either::Right(post) => post,
        };

        p.add_tag(
            &conn,
            &TagEntity::by_label(&conn, "hollow").unwrap().unwrap().id,
        )
        .unwrap();

        if i % 2 == 0 {
            p.add_tag(
                &conn,
                &TagEntity::by_label(&conn, "even").unwrap().unwrap().id,
            )
            .unwrap();
        } else {
            p.add_tag(
                &conn,
                &TagEntity::by_label(&conn, "odd").unwrap().unwrap().id,
            )
            .unwrap();
        }
    }

    // create 1 deleted post
    let deleted_minima = PostMinima {
        author_id: author.id,
        title: "Deleted post".to_string(),
        content: lib::lorem_ipsum(),
        kind: 0,
    };
    let deleted_post = PostEntity::insert_new(&conn, &deleted_minima).unwrap();
    deleted_post
        .add_tag(
            &conn,
            &TagEntity::by_label(&conn, "hollow").unwrap().unwrap().id,
        )
        .unwrap();
    deleted_post.delete(&conn).unwrap();

    // create 1 hidden post
    let hidden_minima = PostMinima {
        author_id: author.id,
        title: "Hidden post".to_string(),
        content: lib::lorem_ipsum(),
        kind: 0,
    };
    let mut hidden_post = PostEntity::insert_new(&conn, &hidden_minima).unwrap();
    hidden_post
        .add_tag(
            &conn,
            &TagEntity::by_label(&conn, "hollow").unwrap().unwrap().id,
        )
        .unwrap();
    hidden_post.toggle_visibility(&conn).unwrap();

    // create 1 locked post
    let locked_minima = PostMinima {
        author_id: author.id,
        title: "Locked post".to_string(),
        content: lib::lorem_ipsum(),
        kind: 0,
    };
    let mut locked_post = PostEntity::insert_new(&conn, &locked_minima).unwrap();
    locked_post
        .add_tag(
            &conn,
            &TagEntity::by_label(&conn, "hollow").unwrap().unwrap().id,
        )
        .unwrap();
    locked_post.toggle_lock(&conn).unwrap();
}

/// Create an author for the posts
fn init_author(conn: &MysqlConnection) -> UserEntity {
    let email = "alan.smithee@unamur.be";
    let activation_token = TokenEntity::create_default(conn).unwrap();
    let recovery_token = TokenEntity::create_default(conn).unwrap();
    let refresh_token = TokenEntity::create_default(conn).unwrap();
    let u = UserMinima {
        email: email.to_string(),
        password: "author".to_string(),
        firstname: "Alan".to_string(),
        lastname: "Smithee".to_string(),
        address: None,
        phone: None,
        activation_token: Some(activation_token.id),
        recovery_token: Some(recovery_token.id),
        refresh_token: Some(refresh_token.id),
    };
    let mut user = UserEntity::insert_either(&conn, &u).unwrap();
    if !user.active {
        user.activate(&conn).unwrap();
    }
    UserEntity::by_email(&conn, email).unwrap().unwrap()
}

fn init_tags(conn: &MysqlConnection) {
    let labels = vec!["even", "odd"];

    for label in labels {
        TagEntity::insert(
            &conn,
            &TagMinima {
                label: label.to_string(),
            },
        )
        .unwrap();
    }
}

use crate::database::models::prelude::{
    PostEntity, PostMinima, TagEntity, TagMinima, UserEntity, UserMinima,
};
use crate::database::Data;
use crate::lib;
use diesel::MysqlConnection;

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
        };

        let p = match PostEntity::insert_minima(&conn, &post_minima) {
            Data::Inserted(post) => post,
            _ => panic!("supposed to be new post"),
        };

        if i % 2 == 0 {
            p.add_tag(&conn, TagEntity::by_label(&conn, "even").unwrap().id);
        } else {
            p.add_tag(&conn, TagEntity::by_label(&conn, "odd").unwrap().id);
        }
    }

    // create 1 deleted post
    let deleted_minima = PostMinima {
        author_id: author.id,
        title: "Deleted post".to_string(),
        content: lib::lorem_ipsum(),
    };
    let deleted_post = match PostEntity::insert_minima(&conn, &deleted_minima) {
        Data::Inserted(p) => p,
        _ => panic!("This should be a new post"),
    };
    deleted_post.delete(&conn);

    // create 1 hidden post
    let hidden_minima = PostMinima {
        author_id: author.id,
        title: "Hidden post".to_string(),
        content: lib::lorem_ipsum(),
    };
    let hidden_post = match PostEntity::insert_minima(&conn, &hidden_minima) {
        Data::Inserted(p) => p,
        _ => panic!("This should be a new post"),
    };
    hidden_post.toggle_visibility(&conn);

    // create 1 locked post
    let locked_minima = PostMinima {
        author_id: author.id,
        title: "Locked post".to_string(),
        content: lib::lorem_ipsum(),
    };
    let locked_post = match PostEntity::insert_minima(&conn, &locked_minima) {
        Data::Inserted(p) => p,
        _ => panic!("This should be a new post"),
    };
    locked_post.toggle_lock(&conn);
}

/// Create an author for the posts
fn init_author(conn: &MysqlConnection) -> UserEntity {
    let email = "alan.smithee@unamur.be";
    let u = UserMinima {
        email: email.to_string(),
        password: "author".to_string(),
        firstname: "Alan".to_string(),
        lastname: "Smithee".to_string(),
        address: None,
        phone: None,
    };
    let user = match UserEntity::insert_minima(&conn, &u) {
        Data::Inserted(u) => u,
        Data::Existing(u) => u,
        _ => panic!("The user is supposed to be a new one"),
    };
    if !user.active {
        user.activate(&conn);
    }
    UserEntity::by_email(&conn, email).unwrap()
}

fn init_tags(conn: &MysqlConnection) {
    let labels = vec!["even", "odd"];

    for label in labels {
        TagEntity::insert(
            &conn,
            &TagMinima {
                label: label.to_string(),
            },
        );
    }
}

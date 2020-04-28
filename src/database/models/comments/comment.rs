use crate::database;
use crate::database::models::prelude::{CommentEntity, User, UserEntity};

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: u32,
    pub post_id: u32,
    pub content: String,
    pub author: User,
    pub created_at: String,
    pub updated_at: String,
    pub votes: u32,
    pub score: i32,
    pub replies: Vec<Comment>,
    pub user_vote: Option<i16>,
}

impl From<CommentEntity> for Comment {
    fn from(ce: CommentEntity) -> Self {
        let conn = database::connection(&database::url());

        // fixme : optimize db request
        Self {
            id: ce.id,
            post_id: ce.post_id,
            content: ce.content.to_string(),
            author: UserEntity::by_id(&conn, ce.author_id)
                .map(|user_entity| User::from(user_entity))
                .unwrap(),
            created_at: ce.created_at.to_string(),
            updated_at: ce.updated_at.to_string(),
            votes: ce.votes,
            score: ce.score,
            replies: vec![],
            // todo : implement replies
            // ce.parent_id
            // .and_then(|comment_id| {
            //     CommentEntity::by_id(&conn, comment_id)
            //         .and_then::<Comment>(|comment_entity| Some(Comment::from(comment_entity)))
            // }),
            user_vote: None,
        }
    }
}

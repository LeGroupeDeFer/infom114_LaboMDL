use crate::database;
use crate::database::models::comments::entity::CommentEntity;
use crate::database::models::posts::tags::entity::RelPostTagEntity;
use crate::database::models::posts::votes::entity::RelPostVoteEntity;
use crate::database::models::prelude::{Comment, PostEntity, User, UserEntity};
use diesel::MysqlConnection;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub locked: bool,
    pub hidden: bool,
    pub votes: u64,
    pub score: i64,
    pub author: User,
    pub tags: Vec<String>,
    pub comments: Vec<Comment>,
    pub user_vote: Option<i16>,
}

impl Post {
    pub fn all(conn: &MysqlConnection) -> Vec<Self> {
        PostEntity::all(conn)
            .drain(..)
            .map(|post_entity| Post::from(post_entity))
            .collect::<Vec<Self>>()
    }

    pub fn set_user_vote(&mut self, conn: &MysqlConnection, user_id: u32) {
        let user_vote =
            RelPostVoteEntity::get(conn, self.id, user_id).map_or(0, |vote| vote.vote_value);

        self.user_vote = Some(user_vote);
    }
}

impl From<PostEntity> for Post {
    fn from(pe: PostEntity) -> Self {
        let conn = database::connection(&database::url());
        // fixme : optimize db request
        Self {
            id: pe.id,
            title: pe.title.to_string(),
            content: pe.content.to_string(),
            created_at: pe.created_at.to_string(),
            updated_at: pe.updated_at.to_string(),
            locked: pe.locked_at.is_some(),
            hidden: pe.hidden_at.is_some(),
            votes: pe.votes,
            score: pe.score,
            author: UserEntity::by_id(&conn, pe.author_id)
                .map(|user_entity| User::from(user_entity))
                .unwrap(),
            tags: RelPostTagEntity::tags_by_post_id(&conn, pe.id)
                .iter()
                .map(|tag_entity| tag_entity.label.to_string())
                .collect::<Vec<String>>(),
            comments: CommentEntity::by_post(&conn, pe.id)
                .drain(..)
                .map(|comment_entity| Comment::from(comment_entity))
                .collect::<Vec<Comment>>(),
            user_vote: None,
        }
    }
}

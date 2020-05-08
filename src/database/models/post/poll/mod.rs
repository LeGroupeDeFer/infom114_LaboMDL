pub mod answers;
pub mod forms;

pub use answers::*;
pub use forms::*;

use crate::database;
use crate::database::models::prelude::{Entity, PostEntity, PostKind};
use crate::lib::{Consequence, Error, PostError};
use diesel::MysqlConnection;
use serde::export::TryFrom;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostPoll {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub answers: Vec<PollAnswer>,
    pub user_answer: Option<PollAnswerEntity>,
}

impl TryFrom<&PostEntity> for PostPoll {
    type Error = Error;

    fn try_from(entity: &PostEntity) -> Consequence<Self> {
        // force the kind of the post to be "poll"
        if u8::from(PostKind::Poll) != entity.kind {
            Err(PostError::InvalidKind)?;
        }

        let conn = database::connection(&database::url());

        let output = Self {
            id: entity.id.clone(),
            title: entity.title.to_string(),
            content: entity.content.to_string(),
            answers: PollAnswer::by_post_id(&conn, &entity.id),
            user_answer: None,
        };

        Ok(output)
    }
}

impl PostPoll {
    pub fn set_user_info(&mut self, conn: &MysqlConnection, user_id: &u32) -> Consequence<()> {
        self.user_answer = PollAnswerEntity::get_user_answer(conn, user_id, &self.id)?;

        Ok(())
    }

    pub fn user_vote(
        &mut self,
        conn: &MysqlConnection,
        user_id: &u32,
        answer_id: &u32,
    ) -> Consequence<()> {
        // first assert that the answer given is linked to this post
        let poll_answer = PollAnswerEntity::by_id(conn, answer_id)??;
        if poll_answer.post_id != self.id {
            Err(PostError::InvalidAnswer)?;
        }

        // then clear every vote for that user to that post
        RelUserPollAnswerEntity::clear_vote_for_user(
            conn,
            user_id,
            self.answers
                .iter()
                .map(|a| &a.id)
                .collect::<Vec<&u32>>()
                .as_ref(),
        )?;

        // then insert the user_id answer_id value
        let minima = RelUserPollAnswerMinima {
            user_id: user_id.clone(),
            answer_id: answer_id.clone(),
        };

        RelUserPollAnswerEntity::insert_new(conn, &minima)?;
        self.answers = PollAnswer::by_post_id(&conn, &self.id);

        self.set_user_info(&*conn, user_id)?;
        Ok(())
    }
}

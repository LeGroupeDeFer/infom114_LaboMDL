use chrono::NaiveDateTime;
use diesel::expression::functions::date_and_time::now;
use diesel::prelude::*;
use diesel::MysqlConnection;
use either::*;

use crate::database::models::Entity;
use crate::lib::consequence::*;

use crate::database::schema::posts;
use crate::database::schema::posts::dsl::{self, posts as table};
use std::hash::{Hasher, Hash};


#[derive(
    Identifiable,
    Queryable,
    AsChangeset,
    Associations,
    Serialize,
    Deserialize,
    Clone,
    Debug,
)]
#[table_name = "posts"]
pub struct PostEntity {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub author_id: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub hidden_at: Option<NaiveDateTime>,
    pub locked_at: Option<NaiveDateTime>,
    pub votes: u64,
    pub score: i64,
    pub rank: f64,
    pub kind: u8,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "posts"]
pub struct PostMinima {
    pub title: String,
    pub content: String,
    pub author_id: u32,
    pub kind: u8
}

impl Entity for PostEntity {
    type Minima = PostMinima;

    fn by_id(conn: &MysqlConnection, id: &u32) -> Consequence<Option<Self>> {
        table
            .find(id)
            .first::<PostEntity>(conn)
            .optional()
            .map(Ok)?
    }

    /// Even if this function is called "all", the posts that are deleted or
    /// hidden are not returned.
    ///
    /// If you want to get the hidden posts too, please use
    /// `PostEntity::admin_all()` method
    fn all(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        let entities = table
            .filter(posts::deleted_at.is_null().and(posts::hidden_at.is_null()))
            .load::<Self>(conn)?;
        Ok(entities)
    }

    /// Since there is no UNIQUE constraint on the minima fields, there is no
    /// `Either::Left` part returned here!
    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        diesel::insert_into(table).values(minima).execute(conn)?;
        let future = Self::select(conn, minima)??;
        Ok(Right(future))
    }

    /// Caution : Since there is no UNIQUE constraint on the minima, it is possible
    /// that the selected entity is not the expected one.
    /// Please use `by_title()`, which explicitly returns a `Vec<Self>`
    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Option<Self>> {
        table
            .filter(
                dsl::title
                    .eq(minima.title.clone())
                    .and(dsl::content.eq(minima.content.clone())),
            )
            .order(dsl::id.desc())
            .first(conn)
            .optional()
            .map(Ok)?
    }

    fn update(&self, conn: &MysqlConnection) -> Consequence<&Self> {
        diesel::update(self)
            .set(self)
            .execute(conn)
            .map(|_| self)
            .map(Ok)?
    }

    fn delete(self, conn: &MysqlConnection) -> Consequence<()> {
        diesel::update(&self)
            .set(dsl::deleted_at.eq(now))
            .execute(conn)
            .map(|_| ())
            .map(Ok)?
    }
}

impl PartialEq for PostEntity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PostEntity {}

impl Hash for PostEntity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
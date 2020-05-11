use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::database::models::prelude::*;
use crate::lib::Consequence;
use either::*;

use crate::database::schema::watch_events;
use crate::database::schema::watch_events::dsl::{self, watch_events as table};

#[derive(Identifiable, Queryable, AsChangeset, Associations, Serialize, Deserialize, Clone, Debug)]
#[table_name = "watch_events"]
#[belongs_to(PostEntity, foreign_key = "post_id")]
#[belongs_to(UserEntity, foreign_key = "author_id")]
pub struct WatchEventEntity {
    pub id: u32,
    pub post_id: u32,
    pub author_id: u32,

    pub event: u8,
    pub time: NaiveDateTime,

    pub comment: String
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "watch_events"]
pub struct WatchEventMinima {
    pub post_id: u32,
    pub author_id: u32,

    pub event: u8,
    pub comment: String
}

impl Entity for WatchEventEntity {
    
    type Minima = WatchEventMinima;

    fn by_id(conn: &MysqlConnection, id: &u32) -> Consequence<Option<Self>> {
        table.find(id).first::<WatchEventEntity>(conn).optional().map(Ok)?
    }

    fn all(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        table.load::<Self>(conn).map(Ok)?
    }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        let past = Self::select(conn, minima)?;
        if past.is_some() {
            Ok(Left(past.unwrap()))
        } else {
            diesel::insert_into(table).values(minima).execute(conn)?;
            let future = Self::select(conn, minima)??;
            Ok(Right(future))
        }
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Option<Self>> {
        table
            .filter(dsl::post_id.eq(minima.post_id.clone()))
            .filter(dsl::author_id.eq(minima.author_id.clone()))
            .filter(dsl::event.eq(minima.event.clone()))
            .first::<Self>(conn)
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
        diesel::delete(table.filter(dsl::id.eq(self.id)))
            .execute(conn)
            .map(|_| ())
            .map(Ok)?
    }
}

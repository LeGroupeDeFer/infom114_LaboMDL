use diesel::prelude::*;
use diesel::MysqlConnection;
use either::*;

use crate::lib::consequence::*;
use crate::database::models::Entity;

use crate::database::schema::tags;
use crate::database::schema::tags::dsl::{self, tags as table};

/* ---------------------------------- Tag ---------------------------------- */
#[derive(Identifiable, Queryable, AsChangeset, Associations, Serialize, Deserialize, Clone, Debug)]
#[primary_key(id)]
#[table_name = "tags"]
pub struct TagEntity {
    pub id: u32,
    pub label: String,
}


#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "tags"]
pub struct TagMinima {
    pub label: String,
}


impl Entity for TagEntity {

    type Minima = TagMinima;

    fn by_id(conn: &MysqlConnection, id: &u32) -> Consequence<Option<Self>> {
        table.find(id).first::<Self>(conn).optional().map(Ok)?
    }

    /// Fetch and return all the roles present in database as a `Role` vector
    fn all(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        table.load(conn).map(Ok)?
    }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Either<Self, Self>> {
        let past = Self::select(conn, minima)?;
        if past.is_some() {
            Ok(Left(past.unwrap()))
        } else {
            diesel::insert_into(table)
                .values(minima.to_lowercase())
                .execute(conn)?;
            let future = Self::select(conn, minima)??;
            Ok(Right(future))
        }
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Consequence<Option<Self>> {
        let label = minima.label.to_lowercase();
        table
            .filter(dsl::label.eq(label))
            .first::<Self>(conn)
            .optional()
            .map(Ok)?
    }

    fn update(&self, conn: &MysqlConnection) -> Consequence<&Self> {
        diesel::update(self).set(self).execute(conn).map(|_| self).map(Ok)?
    }

    fn delete(self, conn: &MysqlConnection) -> Consequence<()>  {
        diesel::delete(table.filter(dsl::id.eq(self.id)))
            .execute(conn)
            .map(|_| ())
            .map(Ok)?
    }

}


impl TagMinima {
    pub fn to_lowercase(&self) -> Self {
        Self {
            label: self.label.to_lowercase(),
        }
    }
}

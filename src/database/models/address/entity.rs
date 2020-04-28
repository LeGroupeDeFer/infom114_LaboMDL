use diesel::prelude::*;
use diesel::MysqlConnection;
use either::*;

use crate::database::schema::addresses;
use crate::database::schema::addresses::dsl::{self, addresses as table};
use crate::database::models::Entity;
use crate::lib::consequence::*;


static DEFAULT_COUNTRY: &str = "BELGIUM";


#[derive(Identifiable, Queryable, AsChangeset, Associations, Serialize, Deserialize, Clone, Debug)]
#[table_name = "addresses"]
pub struct AddressEntity {
    pub id: u32,
    pub street: String,
    pub number: u32,
    pub box_number: Option<String>,
    pub city: String,
    pub zipcode: String,
    pub country: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[table_name = "addresses"]
pub struct AddressMinima {
    pub street: String,
    pub number: u32,
    pub box_number: Option<String>,
    pub city: String,
    pub zipcode: String,
    pub country: Option<String>,
}


impl Entity for AddressEntity {

    type Minima = AddressMinima;

    /* ------------------------------- STATIC ------------------------------ */

    fn by_id(conn: &MysqlConnection, id: &u32) -> Consequence<Option<Self>> {
        table.find(id).first::<AddressEntity>(conn).optional().map(Ok)?
    }

    fn all(conn: &MysqlConnection) -> Consequence<Vec<Self>> {
        table.load(conn).map(Ok)?
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
        let country = &(minima.country).as_deref().unwrap_or(DEFAULT_COUNTRY);

        let condition =
            dsl::street.eq(&minima.street)
            .and(dsl::number.eq(&minima.number))
            .and(dsl::city.eq(&minima.city))
            .and(dsl::zipcode.eq(&minima.zipcode))
            .and(dsl::country.eq(country));

        match &minima.box_number {
            None => table.filter(
                condition.and(dsl::box_number.is_null())
            ).first::<AddressEntity>(conn).optional(),
            Some(bn) => table.filter(
                condition.and(dsl::box_number.eq(bn))
            ).first::<AddressEntity>(conn).optional()
        }.map(Ok)?
    }

    fn update(&self, conn: &MysqlConnection) -> Consequence<&Self> {
        diesel::update(self).set(self).execute(conn).map(|_| self).map(Ok)?
    }

    fn delete(self, conn: &MysqlConnection) -> Consequence<()> {
        diesel::delete(table.filter(dsl::id.eq(self.id)))
            .execute(conn)
            .map(|_| ())
            .map(Ok)?
    }

}

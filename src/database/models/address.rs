use crate::database::schema::addresses;
use crate::database::schema::addresses::dsl::addresses as table;
use super::Entity;
use super::result::*;

use diesel::prelude::*;
use diesel::MysqlConnection;
use either::*;


static DEFAULT_COUNTRY: &str = "BELGIUM";


#[derive(Identifiable, Queryable, AsChangeset, Associations, Serialize, Deserialize, Clone, Debug)]
#[table_name = "addresses"]
pub struct Address {
    pub id: u32,
    pub street: String,
    pub number: u32,
    pub box_number: Option<String>,
    pub city: String,
    pub zipcode: u32,
    pub country: String,
}

impl Entity for Address {

    type Minima = AddressMinima;

    fn of(conn: &MysqlConnection, id: &u32) -> Result<Option<Self>> {
        table.find(id).first::<Address>(conn).optional().map(Ok)?
    }

    fn all(conn: &MysqlConnection) -> Result<Vec<Self>> {
        table.load(conn).map(Ok)?
    }

    fn select(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Option<Self>> {
        let country = &(minima.country).as_deref().unwrap_or(DEFAULT_COUNTRY);

        let condition =
            addresses::street.eq(&minima.street)
            .and(addresses::number.eq(&minima.number))
            .and(addresses::city.eq(&minima.city))
            .and(addresses::zipcode.eq(&minima.zipcode))
            .and(addresses::country.eq(country));

        match &minima.box_number {
            None => table.filter(
                condition.and(addresses::box_number.is_null())
            ).first::<Address>(conn).optional(),
            Some(bn) => table.filter(
                condition.and(addresses::box_number.eq(bn))
            ).first::<Address>(conn).optional()
        }.map(Ok)?
    }

    fn insert(conn: &MysqlConnection, minima: &Self::Minima) -> Result<Either<Self, Self>> {
        let past = Self::select(conn, minima)?;
        if past.is_some() {
            Ok(Left(past.unwrap()))
        } else {
            diesel::insert_into(table).values(minima).execute(conn)?;
            let future = Self::select(conn, minima)??;
            Ok(Right(future))
        }
    }

    fn update(&self, conn: &MysqlConnection) -> Result<&Self> {
        diesel::update(table).set(self).execute(conn).map(|_| self).map(Ok)?
    }

    fn delete(self, conn: &MysqlConnection) -> Result<()> {
        use crate::database::schema::addresses::dsl::id;
        diesel::delete(table.filter(id.eq(self.id))).execute(conn).map(|_| ()).map(Ok)?
    }

}


#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[table_name = "addresses"]
pub struct AddressMinima {
    pub street: String,
    pub number: u32,
    pub box_number: Option<String>,
    pub city: String,
    pub zipcode: u32,
    pub country: Option<String>,
}

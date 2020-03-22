use crate::database::schema::addresses;
use crate::database::schema::addresses::dsl::addresses as table;
use crate::database::Connection;
use diesel::prelude::*;
use either::*;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, Debug)]
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

impl Address {
    /* ------------------------------- STATIC ------------------------------ */

    pub fn from(conn: &Connection, id: &u32) -> Option<Self> {
        table.find(id).first::<Address>(&**conn).ok()
    }

    pub fn all(conn: &Connection) -> Vec<Self> {
        table.load(&**conn).unwrap_or(vec![])
    }

    // select_minima :: (Connection, AddressMinima) -> Option<Address>
    pub fn select_minima(conn: &Connection, minima: &AddressMinima) -> Option<Self> {
        table
            .filter(addresses::street.eq(&minima.street))
            .filter(addresses::number.eq(&minima.number))
            .filter(addresses::box_number.eq(&minima.box_number))
            .filter(addresses::city.eq(&minima.city))
            .filter(addresses::zipcode.eq(&minima.zipcode))
            .filter(addresses::country.eq(&minima.country))
            .first::<Address>(&**conn)
            .ok()
    }

    // insert_minima :: (Connection, AddressMinima) -> Either<Address, Address>
    pub fn insert_minima(conn: &Connection, minima: &AddressMinima) -> Either<Self, Self> {
        if let Some(past) = Address::select_minima(conn, minima) {
            Left(past)
        } else {
            diesel::insert_into(table)
                .values(minima)
                .execute(&**conn)
                .expect("Failed address insertion");
            Right(
                Address::select_minima(conn, minima)
                    .expect("Address insertion succeeded but could not be retreived"),
            )
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "addresses"]
pub struct AddressMinima {
    pub street: String,
    pub number: u32,
    pub box_number: Option<String>,
    pub city: String,
    pub zipcode: u32,
    pub country: String,
}

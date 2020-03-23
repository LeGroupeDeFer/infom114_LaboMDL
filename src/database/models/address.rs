use crate::database::schema::addresses;
use crate::database::schema::addresses::dsl::addresses as table;
use crate::database::DBConnection;
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

    // from :: (DBConnection, Integer) -> Option<Address>
    pub fn from(conn: &DBConnection, id: &u32) -> Option<Self> {
        table.find(id).first::<Address>(&**conn).ok()
    }

    // all :: (DBConnection, Integer) -> Option<Address>
    pub fn all(conn: &DBConnection) -> Vec<Self> {
        table.load(&**conn).unwrap_or(vec![])
    }

    /// Get the address record that fits the `minima` given.
    pub fn select_minima(conn: &DBConnection, minima: &AddressMinima) -> Option<Self> {
        let filtered = table.filter(
            addresses::street
                .eq(&minima.street)
                .and(addresses::number.eq(&minima.number))
                .and(addresses::city.eq(&minima.city))
                .and(addresses::zipcode.eq(&minima.zipcode))
                .and(addresses::country.eq(&minima.country)),
        );

        // Since a rust `None` value is not equal to a SQL `NULL` value, a custom test must be
        // performed to correctly identify the address.
        match &minima.box_number {
            None => filtered
                .filter(addresses::box_number.is_null())
                .first::<Address>(&**conn)
                .ok(),
            Some(box_n) => filtered
                .filter(
                    addresses::box_number
                        .is_not_null()
                        .and(addresses::box_number.eq(box_n)),
                )
                .first::<Address>(&**conn)
                .ok(),
        }
    }

    // insert_minima :: (DBConnection, AddressMinima) -> Either<Address, Address>
    pub fn insert_minima(conn: &DBConnection, minima: &AddressMinima) -> Either<Self, Self> {
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

use crate::database::schema::addresses;
use crate::database::tables::addresses_table as table;
use diesel::prelude::*;
use diesel::MysqlConnection;
use either::*;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Clone, Debug)]
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

impl AddressEntity {
    /* ------------------------------- STATIC ------------------------------ */

    // from :: (MysqlConnection, Integer) -> Option<Address>
    pub fn by_id(conn: &MysqlConnection, id: &u32) -> Option<Self> {
        table.find(id).first::<Self>(conn).ok()
    }

    // all :: (MysqlConnection, Integer) -> Option<Address>
    pub fn all(conn: &MysqlConnection) -> Vec<Self> {
        table.load(conn).unwrap_or(vec![])
    }

    /// Get the address record that fits the `minima` given.
    pub fn select_minima(conn: &MysqlConnection, minima: &AddressMinima) -> Option<Self> {
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
                .first::<Self>(conn)
                .ok(),
            Some(box_n) => filtered
                .filter(
                    addresses::box_number
                        .is_not_null()
                        .and(addresses::box_number.eq(box_n)),
                )
                .first::<Self>(conn)
                .ok(),
        }
    }

    // insert_minima :: (MysqlConnection, AddressMinima) -> Either<Address, Address>
    pub fn insert_minima(conn: &MysqlConnection, minima: &AddressMinima) -> Either<Self, Self> {
        if let Some(past) = Self::select_minima(conn, minima) {
            Left(past)
        } else {
            diesel::insert_into(table)
                .values(minima)
                .execute(conn)
                .expect("Failed address insertion");
            Right(
                Self::select_minima(conn, minima)
                    .expect("Address insertion succeeded but could not be retrieved"),
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
    pub zipcode: String,
    pub country: String,
}

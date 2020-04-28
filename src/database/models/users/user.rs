use crate::database;
use crate::database::models::prelude::{Address, AddressEntity, Role, UserEntity};
use crate::database::models::users::roles::entity::RelUserRoleEntity;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub address: Option<Address>,
    pub roles: Vec<Role>,
}

impl From<UserEntity> for User {
    fn from(ue: UserEntity) -> Self {
        let conn = database::connection(&database::url());
        Self {
            id: ue.id,
            firstname: ue.firstname.to_string(),
            lastname: ue.lastname.to_string(),
            email: ue.email.to_string(),
            address: ue.address.and_then(|address_id| {
                AddressEntity::by_id(&conn, address_id)
                    .and_then(|address_entity| Some(Address::from(address_entity)))
            }),
            roles: RelUserRoleEntity::get_roles_by_user(&conn, &ue)
                .drain(..)
                .map(|role_entity| Role::from(role_entity))
                .collect::<Vec<Role>>(),
        }
    }
}

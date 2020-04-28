/// The `UserRoleData` struct is used when a role needs to be assigned to
/// a user, or when the decision to unassign the role from the user is taken.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserRoleData {
    pub user_id: u32,
    pub role_id: u32,
}

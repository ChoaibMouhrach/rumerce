pub struct Role {
    pub admin: &'static str,
    pub member: &'static str,
}

pub const ROLES: Role = Role {
    admin: "admin",
    member: "member",
};

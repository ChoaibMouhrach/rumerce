pub struct Role {
    pub admin: &'static str,
    pub member: &'static str,
}

pub const ROLES: Role = Role {
    admin: "admin",
    member: "member",
};

pub const SESSION_COOKIE_NAME: &str = "session";
pub const CART_COOKIE_NAME: &str = "cart";

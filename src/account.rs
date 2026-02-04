/// A generated MEGA account.
#[derive(Debug, Clone)]
pub struct GeneratedAccount {
    /// The email address used for registration.
    pub email: String,
    /// The account password.
    pub password: String,
    /// The account holder's name.
    pub name: String,
}

impl std::fmt::Display for GeneratedAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Email: {}\nPassword: {}\nName: {}",
            self.email, self.password, self.name
        )
    }
}

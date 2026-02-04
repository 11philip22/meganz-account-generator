/// Credentials returned after successful account generation and confirmation.
///
/// The `password` field is the same plaintext value passed to
/// [`crate::AccountGenerator::generate`] or
/// [`crate::AccountGenerator::generate_with_name`].
#[derive(Debug, Clone)]
pub struct GeneratedAccount {
    /// Temporary email address used for registration.
    pub email: String,
    /// Account password provided by the caller.
    pub password: String,
    /// Account display name used during signup.
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

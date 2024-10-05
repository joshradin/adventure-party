use naia_bevy_shared::Message;

#[derive(Message)]
pub struct Auth {
    username: String,
    password: String,
}

impl Auth {
    /// Creates a new auth message
    pub fn new(username: impl AsRef<str>, password: impl AsRef<str>) -> Self {
        Self {
            username: username.as_ref().to_string(),
            password: password.as_ref().to_string(),
        }
    }
}

use argon2::{self, Config, Variant, Version};
/// Password hashing utility using Argon2
pub struct PasswordHasher {
    salt:String, // Example salt, should be generated securely
    config: Config<'static>,
}
/// Implementing PasswordHasher  struct
impl PasswordHasher {
    /// Creates a new PasswordHasher with default configuration
    pub fn new() -> Self {
        Self {
            salt: "sunny9807".into(),
            config: Config {
                variant: Variant::Argon2i,
                version: Version::Version13,
                mem_cost: 65536,
                time_cost: 10,
                lanes: 4,
                secret: &[],
                ad: &[],
                hash_length: 32
            },
        }
    }
    /// Creates a new PasswordHasher with a custom salt
    pub fn hash(&self, password: &str) -> String {
        let hash = argon2::hash_encoded(password.as_bytes(), &self.salt.as_bytes(), &self.config).unwrap();
        hash
    }
    /// Verifies a password against an encoded hash
    pub fn verify(&self, password: &str, encoded_hash: &str) -> bool {
        argon2::verify_encoded(encoded_hash, password.as_bytes()).unwrap_or(false)
    }
}
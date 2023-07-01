use argon2::{self, Config};

#[doc = "Salt and hash a password"]
pub fn hash_password(password: &str) -> String {
  let config = Config::default();
  let salt = b"mermoauthhash";
  
  let hash = argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap_or_else(|_| String::new());
  
  hash
}

#[doc = "Verify a password against a hash"]
pub fn verify_password(password: &str, hashed_password: &str) -> bool {
  argon2::verify_encoded(hashed_password, password.as_bytes()).unwrap_or(false)
}
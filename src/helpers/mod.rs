use core::panic;
use std::env;

use argon2rs::{
    Argon2,
    defaults::{KIB, LANES, PASSES}, Variant,
};
use argon2rs::verifier::Encoded;
use mysql::*;
use rand::{Rng, thread_rng};

/// read_env returns the value of a dotenv variable.
/// in case the variable is not set, the application panics.
/// Note, this function returns a String instance, in case you need to use &str,
/// convert
///
/// # Examples
///
/// ```
/// let port:String = read_env("PORT");
/// ```
pub fn read_env(key: &str) -> String {
    match env::var(key) {
        Ok(v) => v,
        Err(_) => {
            panic!("key {} was not configured", key)
        }
    }
}

/// init_logger initiates the logger depending on the environment.
/// if the environment is dev, then the log level is set to DEBUG and the
/// output is os.Stdout
/// Otherwise, the log level is INFO and written to a file and rotated.
pub fn init_logger() {
    let env_value = read_env("ENV");
    match env_value {
        // by default, use dev configs
        _ => {
            env::set_var("RUST_LOG", "debug");
            env_logger::init()
        }
    }
}

/// connect_to_db returns a db connection pool to mysql.
/// make sure to get a connection instance from the pool.
///
/// # Examples
///
/// ```
/// let pool = connect_to_db()
/// ```
pub fn connect_to_db() -> Pool {
    match Pool::new(&read_env("DB_URI")[..]) {
        Ok(p) => p,
        Err(e) => {
            panic!("unable to connect to mysql because {:?}", e);
        }
    }
}

pub fn hash_password(clear_text: &String) -> (String, String) {
    let salt = read_env("SALT");
    let random_salt = thread_rng().gen_ascii_chars().take(32).collect::<String>();

    let a2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2d).unwrap();
    let random_salt_hash =
        Encoded::new(a2, random_salt.as_bytes(), salt.as_bytes(), b"", b"").to_u8();
    let random_salt_hash_storable_encoding = String::from_utf8(random_salt_hash).unwrap();

    let a2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2d).unwrap();
    let data_hash = Encoded::new(
        a2,
        clear_text.as_bytes(),
        random_salt_hash_storable_encoding.as_bytes(),
        b"",
        b"",
    )
        .to_u8();
    let password_hash = String::from_utf8(data_hash).unwrap();

    (random_salt, password_hash)
}

pub fn verify_hash(clear_text: String, password_hash: String, stored_salt: String) -> bool {
    let salt = read_env("SALT");

    let a2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2d).unwrap();
    let random_salt_hash =
        Encoded::new(a2, stored_salt.as_bytes(), salt.as_bytes(), b"", b"").to_u8();
    let random_salt_hash_storable_encoding = String::from_utf8(random_salt_hash).unwrap();

    let a2 = Argon2::new(PASSES, LANES, KIB, Variant::Argon2d).unwrap();
    let data_hash = Encoded::new(
        a2,
        clear_text.as_bytes(),
        random_salt_hash_storable_encoding.as_bytes(),
        b"",
        b"",
    )
        .to_u8();
    let hashed = String::from_utf8(data_hash).unwrap();
    hashed.eq(&password_hash)
}

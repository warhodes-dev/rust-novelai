use blake2::{Blake2b, Digest, digest::consts::U16};
use crate::Result;
use crate::error::Error;

/// Generates an access key from email and password.
///
/// Access key is a Base64 encoded string, truncated to 64 chars.
///
/// Mainly for use with the login api call.
pub fn get_access_key(email: &str, password: &str) -> Result<String> {
    let hash = argon_hash(email, password, 64, "novelai_data_access_key")?;
    let full_key = base64::encode_config(hash, base64::URL_SAFE);
    Ok(full_key.get(0..64)
        .expect("Generated Base64 key is invalid.")
        .to_owned())
}

/// Generate an encryption key from email and password.
///
/// Encryption key is a Base64 encoded string.
pub fn get_encryption_key(email: &str, password: &str) -> Result<Vec::<u8>> {
    argon_hash(email, password, 128, "novelai_data_encryption_key")
}

/// Note: password length must be at least 6
fn argon_hash(
    email: &str, 
    password: &str, 
    size: u32, 
    domain: &str
) -> Result<Vec<u8>> {

    if password.len() < 6 {
        return Err(Error::BadPassword);
    }

    let pre_salt = format!("{}{email}{domain}", &password[0..6]);
    let mut hasher = Blake2b::<U16>::new();
    hasher.update(pre_salt.as_bytes());
    let salt = hasher.finalize();
    let config = argon2::Config {
        variant: argon2::Variant::Argon2id,
        version: argon2::Version::Version13,
        mem_cost: 2000000 / 1024,
        time_cost: 2,
        hash_length: size,
        ..Default::default()
    };

    //TODO: Possibly handle this error? Or just leave it as a panic.
    Ok(argon2::hash_raw(password.as_bytes(), &salt, &config)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_keys() {
        let email = "testuser@email.com";
        let password = "123secret456";
        
        // Generated from the same email and pasword as above using the novelai-api python lib
        let valid_key = "3MRZSCsV2I4eqwKmOYKmWIPkF5_TEbjcI-JLfvRgeYlU8_E27DJxVfNSUAPw0nwo";
        assert_eq!(get_access_key(email, password).unwrap(), valid_key);
    }

    /*
    #[test]
    fn user_is_crazy() {
        let email = "brad. just brad.";
        let password = "rust";

        // We don't know why or how python allowed this to work, but it did.
        match get_access_key(email, password) {
            Err(e) => {
                assert!(e.is::<CryptoError>())
            },
            _ => panic!("Didn't catch passwordlength error"),
        }
    }
    */
}

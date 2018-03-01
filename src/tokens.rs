use crypto::sha2::Sha256;
use crypto::digest::Digest;
use jwt::{Algorithm,Validation,Header,encode,decode};
use jwt::errors::Error;

use env;
use models::User;

// Claims found here:
// https://github.com/standardfile/rails-engine/blob/ad43d5761c01b4a3eba00d525c9a76a051491d27/lib/standard_file/user_manager.rb#L79
#[derive(Debug,Serialize,Deserialize)]
pub struct Claims {
    pub user_uuid: String,
    pub pw_hash: String,
}

pub fn user_to_jwt(user: &User) -> Result<String,Error> {
   encode(&Header::default(),
           &Claims {
               user_uuid: user.uuid.to_owned(),
               pw_hash: sha256(user.encrypted_password.as_str()),
           },
           env::get_secret_key().as_bytes())
}

pub fn decode_jwt(token: &String) -> Result<Claims,Error> {
    match decode::<Claims>(token,
                           env::get_secret_key().as_bytes(),
                           &Validation::new(Algorithm::HS256)) {
        Ok(data) => Ok(data.claims),
        Err(err) => Err(err)
    }
}

pub fn sha256(input: &str) -> String {
    let mut hash = Sha256::new();
    hash.input_str(input);
    hash.result_str()
}
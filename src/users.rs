use uuid::Uuid;
use chrono::{DateTime, UTC};

#[derive(Debug,PartialEq,Eq)]
pub struct SFUser {
    uuid: Uuid,

    encrypted_password: String,
    pw_func: String,
    pw_alg: String,
    pw_cost: i32,
    pw_key_size: i32,
    pw_nonce: String,
    email: String,
    created_at: DateTime<UTC>,
    updated_at: DateTime<UTC>,
}

pub fn create_new(email: String,
                  password: String,
                  pw_func: String,
                  pw_alg: String,
                  pw_cost: i32,
                  pw_key_size: i32,
                  pw_nonce: String)
                  -> SFUser {
    let cur_time = UTC::now();

    SFUser {
        // ruby-server uses SecureRandom.uuid
        uuid: Uuid::new_v4(),
        encrypted_password: password,
        pw_func: pw_func,
        pw_alg: pw_alg,
        pw_cost: pw_cost,
        pw_key_size: pw_key_size,
        pw_nonce: pw_nonce,
        email: email,
        created_at: cur_time,
        updated_at: cur_time,
    }
}

/**
 * Non-mutable update.
 */
pub fn update(newpass: String, user: &SFUser) -> SFUser {
    let cur_time = UTC::now();

    SFUser {
        uuid: user.uuid.clone(),
        pw_func: user.pw_func.clone(),
        pw_alg: user.pw_alg.clone(),
        pw_cost: user.pw_cost.clone(),
        pw_key_size: user.pw_key_size.clone(),
        pw_nonce: user.pw_nonce.clone(),
        email: user.email.clone(),
        created_at: user.created_at.clone(),

        encrypted_password: newpass,
        updated_at: cur_time,
    }
}

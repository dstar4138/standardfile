use uuid::Uuid;
use chrono::{DateTime, UTC};

#[derive(Debug,PartialEq,Eq)]
pub struct SFUser {
    pub uuid: Uuid,

    pub encrypted_password: String,
    pub pwd: PWDetails,
    pub email: String,
    pub created_at: DateTime<UTC>,
    pub updated_at: DateTime<UTC>,
}
#[derive(Serialize,Deserialize,Debug,PartialEq,Eq)]
pub struct PWDetails {
    pub pw_func: String,
    pub pw_alg: String,
    pub pw_cost: i32,
    pub pw_key_size: i32,
    pub pw_nonce: String,
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
        pwd: PWDetails {
            pw_func: pw_func,
            pw_alg: pw_alg,
            pw_cost: pw_cost,
            pw_key_size: pw_key_size,
            pw_nonce: pw_nonce,
        },
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
        pwd: get_pw_details(&user),
        email: user.email.clone(),
        created_at: user.created_at.clone(),

        encrypted_password: newpass,
        updated_at: cur_time,
    }
}

pub fn get_pw_details(user: &SFUser) -> PWDetails {
    PWDetails {
        pw_func: user.pwd.pw_func.clone(),
        pw_alg: user.pwd.pw_alg.clone(),
        pw_cost: user.pwd.pw_cost.clone(),
        pw_key_size: user.pwd.pw_key_size.clone(),
        pw_nonce: user.pwd.pw_nonce.clone(),
    }
}
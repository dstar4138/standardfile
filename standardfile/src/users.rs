use util;
use backend_core::models::User;
use pwdetails::PasswordDetails;

pub fn create_new(
    email:String,
    encrypted_password:String,
    pwd: PasswordDetails
) -> User {
    let current_time = util::current_time();
    User{
        email,
        encrypted_password,

        uuid: util::new_uuid(),
        created_at: current_time,
        updated_at: current_time,

        pw_func:     pwd.pw_func,
        pw_alg:      pwd.pw_alg,
        pw_cost:     pwd.pw_cost,
        pw_key_size: pwd.pw_key_size,
        pw_nonce:    pwd.pw_nonce,
        pw_salt:     pwd.pw_salt,
        version:     pwd.version,
    }
}

/**
 * Non-mutable update.
 */
pub fn update(new_pass: String, user: User) -> User {
    use util;
    let current_time = util::current_time();
    User{
        updated_at: current_time,
        encrypted_password: new_pass,
        ..user
    }
}
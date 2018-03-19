use util;
use backend_core::models::User;
use pwdetails::{PasswordDetails,HasPasswordDetails};

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

        pw_func:     pwd.get_pw_func(),
        pw_alg:      pwd.get_pw_alg(),
        pw_cost:     pwd.get_pw_cost(),
        pw_key_size: pwd.get_pw_key_size(),
        pw_nonce:    pwd.get_pw_nonce(),
        pw_salt:     pwd.get_pw_salt(),
        version:     pwd.get_version(),
    }
}

impl HasPasswordDetails for User {
    fn get_pw_func(&self) -> String {
        self.pw_func.clone()
    }
    fn get_pw_alg(&self) -> String {
        self.pw_alg.clone()
    }
    fn get_pw_cost(&self) -> i32 {
        self.pw_cost.clone()
    }
    fn get_pw_key_size(&self) -> i32 {
        self.pw_key_size.clone()
    }
    fn get_pw_nonce(&self) -> String {
        self.pw_nonce.clone()
    }
    fn get_pw_salt(&self) -> String {
        self.pw_salt.clone()
    }
    fn get_version(&self) -> String {
        self.version.clone()
    }
    fn to_password_details(&self) -> PasswordDetails {
        PasswordDetails {
            pw_func:    Some(self.pw_func.clone()),
            pw_alg:     Some(self.pw_alg.clone()),
            pw_cost:    Some(self.pw_cost.clone()),
            pw_key_size:Some(self.pw_key_size.clone()),
            pw_nonce:   Some(self.pw_nonce.clone()),
            pw_salt:    Some(self.pw_salt.clone()),
            version:    Some(self.version.clone()),
        }
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
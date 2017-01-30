use uuid::Uuid;
use rusqlite::Connection;

// Types.
use items;
use users;

pub fn create_connection(path: &str) -> Connection {
    let conn = Connection::open(path).unwrap();
    verify_tables(&conn);
    conn
}
fn verify_tables(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
                uuid                VARCHAR(36) NOT NULL PRIMARY KEY,
                encrypted_password  VARCHAR(255) DEFAULT \"\" NOT NULL, 
                pw_func             VARCHAR(255),
                pw_alg              VARCHAR(255),
                pw_cost             INTEGER,
                pw_key_size         INTEGER,
                pw_nonce            VARCHAR(255),
                email               VARCHAR(255) NOT NULL UNIQUE,
                created_at          DATETIME NOT NULL,
                updated_at          DATETIME NOT NULL
        )", &[]).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS items (
                uuid                VARCHAR(36) NOT NULL PRIMARY KEY,
                content             VARCHAR(65535),
                content_type        VARCHAR(255),
                enc_item_key        VARCHAR(255),
                auth_hash           VARCHAR(255),
                user_uuid           VARCHAR(36) NOT NULL,
                created_at          DATETIME NOT NULL,
                updated_at          DATETIME NOT NULL,
                deleted             BOOLEAN DEFAULT FALSE
        )", &[]).unwrap();
}

pub fn add_user(conn: &Connection, user: users::SFUser) -> () {
    conn.execute(
        "INSERT INTO users (
            uuid,
            encrypted_password,
            pw_func,
            pw_alg,
            pw_cost,
            pw_key_size,
            pw_nonce,
            email,
            created_at,
            updated_at
         ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
         &[&user.uuid.urn().to_string(),
           &user.encrypted_password,
           &user.pw_func,
           &user.pw_alg,
           &user.pw_cost,
           &user.pw_key_size,
           &user.pw_nonce,
           &user.email,
           &user.created_at,
           &user.updated_at
          ]).unwrap();
}

pub fn add_item(conn: &Connection, item: items::SFItem) -> () {
    conn.execute(
        "INSERT INTO items (
            uuid,
            content,
            content_type,
            enc_item_key,
            auth_hash,
            user_uuid,
            created_at,
            updated_at,
            deleted
         ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
        &[&item.uuid.urn().to_string(),
          &item.content,
          &item.content_type,
          &item.enc_item_key,
          &item.auth_hash,
          &item.user_uuid.urn().to_string(),
          &item.created_at,
          &item.updated_at,
          &item.deleted
         ]).unwrap();
}

pub fn update_item(conn: &Connection, item: items::SFItem) -> () {
    conn.execute(
        "UPDATE items SET 
            content = ?2,
            content_type = ?3,
            enc_item_key = ?4,
            auth_hash = ?5,
            updated_at = ?6,
            deleted = ?7
         WHERE uuid = ?1 LIMIT 1",
         &[&item.uuid.urn().to_string(),
           &item.content,
           &item.content_type,
           &item.enc_item_key,
           &item.auth_hash,
           &item.updated_at,
           &item.deleted
         ]).unwrap();
}

pub fn find_user_by_email(conn: &Connection, email: String) -> Option<users::SFUser> {
    let mut stmt = conn.prepare(
        "SELECT 
            uuid,
            encrypted_password,
            pw_func,
            pw_alg,
            pw_cost,
            pw_key_size,
            pw_nonce,
            email,
            created_at,
            updated_at
         FROM users WHERE email = ? LIMIT 1").unwrap();
    let mut rows = stmt.query(&[ &email ]).unwrap();
    match rows.next() {
        None => None,
        Some(Err(_)) => None,
        Some(Ok(row)) => { 
            let id_str: String = row.get(0);
            Some(users::SFUser {
                uuid: Uuid::parse_str(id_str.as_str()).unwrap(),
                encrypted_password:   row.get(1),
                pw_func:              row.get(2),
                pw_alg:               row.get(3),
                pw_cost:              row.get(4),
                pw_key_size:          row.get(5),
                pw_nonce:             row.get(6),
                email:                row.get(7),
                created_at:           row.get(8),
                updated_at:           row.get(9)
            })
        }
    }
}

pub fn get_items_by_user_uuid(conn: &Connection, user_uuid: &Uuid) -> Option<Vec<items::SFItem>> {
    let mut stmt = conn.prepare(
        "SELECT
            uuid,
            user_uuid,
            content,
            content_type,
            enc_item_key,
            auth_hash,
            created_at,
            updated_at,
            deleted
         FROM items WHERE user_uuid = ?").unwrap();
    let mut rows = stmt.query(&[&user_uuid.urn().to_string()]).unwrap();
    let mut items = Vec::new();
    while let Some(wrap_row) = rows.next() {
        let row = wrap_row.unwrap();

        let id_str: String = row.get(0);
        let uid_str: String = row.get(1);
        items.push(
            items::SFItem {
                uuid     : Uuid::parse_str(id_str.as_str()).unwrap(),
                user_uuid: Uuid::parse_str(uid_str.as_str()).unwrap(),
                content      : row.get(2),
                content_type : row.get(3),
                enc_item_key : row.get(4),
                auth_hash    : row.get(5),
                created_at   : row.get(6),
                updated_at   : row.get(7),
                deleted      : row.get(8)
            });
    }

    Some(items)
}

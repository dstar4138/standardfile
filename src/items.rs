use uuid::Uuid;
use chrono::{DateTime, UTC};

#[derive(Debug,PartialEq,Eq)]
pub struct SFItem {
    uuid: Uuid,

    content:      String, // Base64
    content_type: String,
    enc_item_key: String, // Base64
    auth_hash:    String, // Hex
    deleted:      bool,    
    created_at:   DateTime<UTC>,
    updated_at:   DateTime<UTC>  
}

pub fn create_new( content     : String, 
                   content_type: String,
                   enc_item_key: String,
                   auth_hash   : String 
                 ) -> SFItem {

    let cur_time = UTC::now();

    SFItem {
        uuid         : Uuid::new_v4(), // ruby-server uses SecureRandom.uuid
        content      : content,
        content_type : content_type,
        enc_item_key : enc_item_key,
        auth_hash    : auth_hash,
        deleted      : false,
        created_at   : cur_time, 
        updated_at   : cur_time
    }
}

/**
 * Non-mutable delete.
 */
pub fn mark_deleted( item: &SFItem ) -> SFItem {
    let cur_time = UTC::now();
    SFItem {
        uuid         : item.uuid,
        created_at   : item.created_at,

        content      : "".to_string(),
        content_type : "".to_string(),
        enc_item_key : "".to_string(),
        auth_hash    : "".to_string(),
        deleted      : true,
        updated_at   : cur_time 
    }
}

/**
 * Non-mutable update.
 */
pub fn update( content     : String, 
               content_type: String,
               enc_item_key: String,
               auth_hash   : String,
               item        : &SFItem ) -> SFItem {
    let cur_time = UTC::now();
    SFItem {
        uuid         : item.uuid,
        created_at   : item.created_at,
        deleted      : item.deleted,

        content      : content,
        content_type : content_type,
        enc_item_key : enc_item_key,
        auth_hash    : auth_hash, 
        updated_at   : cur_time 
    }
}


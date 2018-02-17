use util;
use models::Item;

pub fn create_new( user_uuid   : String,
                   content     : String, 
                   content_type: String,
                   enc_item_key: String,
                   auth_hash   : String,
                   last_user_agent : String
                 ) -> Item {
    let cur_time = util::current_time();
    Item {
        uuid         : util::new_uuid(),
        user_uuid,
        content: content.into_bytes(),
        content_type,
        enc_item_key,
        auth_hash,
        last_user_agent,
        deleted      : false,
        created_at   : cur_time, 
        updated_at   : cur_time
    }
}

/**
 * Non-mutable delete.
 */
pub fn mark_deleted( item: &Item ) -> Item {
    let cur_time = util::current_time();
    Item {
        uuid         : item.uuid.clone(),
        user_uuid    : item.user_uuid.clone(),
        created_at   : item.created_at.clone(),

        content      : "".to_string().into_bytes(),
        content_type : "".to_string(),
        enc_item_key : "".to_string(),
        auth_hash    : "".to_string(),
        last_user_agent : "".to_string(),

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
               last_user_agent : String,
               item        : &Item ) -> Item {
    let cur_time = util::current_time();
    Item {
        uuid         : item.uuid.clone(),
        user_uuid    : item.user_uuid.clone(),
        created_at   : item.created_at.clone(),
        deleted      : item.deleted.clone(),

        content: content.into_bytes(),
        content_type,
        enc_item_key,
        auth_hash,
        last_user_agent,
        updated_at   : cur_time 
    }
}


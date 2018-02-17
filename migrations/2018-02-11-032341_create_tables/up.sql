CREATE TABLE IF NOT EXISTS "items" (
    "uuid" VARCHAR(36) PRIMARY KEY NULL,
    "content" BLOB NOT NULL,
    "content_type" VARCHAR(512) NOT NULL,
    "enc_item_key" VARCHAR(65535) NOT NULL,
    "auth_hash" VARCHAR(512) NOT NULL,
    "user_uuid" VARCHAR(36) NOT NULL,
    "deleted" INTEGER(1) NOT NULL DEFAULT 0,
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "last_user_agent" VARCHAR(65535)
);
CREATE TABLE IF NOT EXISTS "users" (
    "uuid" VARCHAR(36) PRIMARY KEY NOT NULL,
    "email" VARCHAR(255) NOT NULL,
    "pw_func" VARCHAR(255) NOT NULL DEFAULT "pbkdf2",
    "pw_alg" VARCHAR(255) NOT NULL DEFAULT "sha512",
    "pw_cost" INTEGER NOT NULL DEFAULT 5000,
    "pw_key_size" INTEGER NOT NULL DEFAULT 512,
    "pw_nonce" VARCHAR(255) NOT NULL,
    "encrypted_password" VARCHAR(255) NOT NULL DEFAULT "",
    "pw_salt" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "version" VARCHAR(255) NOT NULL
);
CREATE INDEX IF NOT EXISTS user_uuid ON items (user_uuid);
CREATE INDEX IF NOT EXISTS user_content ON items (user_uuid, content_type);
CREATE INDEX IF NOT EXISTS updated_at ON items (updated_at);
CREATE INDEX IF NOT EXISTS email ON users (email);
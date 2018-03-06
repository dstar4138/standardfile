# StandardFile API v0.0.2

## POST /auth

Registers a user with the given configurations and returns a JWT to authorize further calls.

### Headers:

None.

### Params:
```json
{
  "email" : "",
  "password" : ""
}
```

#### Optional Parameters:

All password details look to be overridable. Here are the current defaults:
```json
{
  "pw_func" : "pbkdf2",
  "pw_alg"  : "sha512",
  "pw_cost" : 5000,
  "pw_key_size": 512,
  "pw_nonce": "", // unused?
  "pw_salt" : "", //sha1 digest of [email, "SN", server's salt value]
  "version" : "002"
}
```

### Responses:

* Success: `200` => `{"token" : "...", "user" : { "uuid":"...", "email":"..." }}`
* Failure: 
  * Missing all params/body: `403` => `{"error": [{"message":"Unable to register.", "status":403}]}`
  * Missing some params: `403` => `{"error": [{"message":Missing valid %s.", "status":403}]}`
    * Replace `%s` with any of: `email`, or `password`.
    * Missing any optional parameters will resort to using default values.
  * User's email already exists: `403` => `{"error": [{"message":"This email is already registered.", "status":403}]}`
  
## POST /auth/sign_in

Confirms a user's email/password hash and then returns a JWT to authorize further calls.

### Headers:

None.

### Params:
```json
{
  "email" : "",
  "password" : ""
}
```

### Responses:

* Success: `200` => `{"token" : "...", "user" : { "uuid":"...", "email":"..." }}`
* Failures: 
  * Missing all params/body: `403` => `{"error": [{"message":"Invalid email or password.", "status":403}]}` 
  * Missing some params: `403` => `{"error": [{"message":"Invalid email or password.", "status":403}]}` 
  * User does not exist: `403` => `{"error": [{"message":"Invalid email or password.", "status":403}]}` 

## GET /auth/params

Returns the parameters to used for password generation on the client.
Returns default parameters if no user present in database.

### Headers:

None.

### Params:

`?email=%s` where `%s` matches the form `.*@.*` (i.e contains an "at" sign).

### Responses:

* Success: `200` =>
```json
{
  "pw_func" : "pbkdf2",
  "pw_alg"  : "sha512",
  "pw_cost" : 5000,
  "pw_key_size": 512,
  "pw_nonce": "", // unused?
  "pw_salt" : "", //sha1 digest of [email, "SN", server's salt value]
  "version" : "002"
}
```
* Failures: 
  * Missing email: `400` => `{"error": [{"message":"Please provide email via GET paramater.", "status":400}]}` 
  * Invalid email: `400` => `{"error": [{"message":"Please provide email via GET paramater.", "status":400}]}` 

## PATCH /auth

Change the user's password and logs the user in with new password. 
Prior JWT becomes invalid.

### Headers:

`Authorization: Bearer _JSON_WEB_TOKEN_`

### Params:
```json
{
  "email" : "", 
  "password" : "", 
  "password_confirmation" : "",
  "current_password" : ""
}
```

#### Optional Params:

This API also allows for password generation parameter updates too. 
If any of the following are provided then the parameter will be updated.

```json
{
  "pw_func" : "pbkdf2",
  "pw_alg"  : "sha512",
  "pw_cost" : 5000,
  "pw_key_size": 512,
  "pw_nonce": "", // unused?
  "pw_salt" : "", //sha1 digest of [email, "SN", server's salt value]
  "version" : "002"
}
```

### Responses:

* Success: `200` => `{"token" : "...", "user" : { "uuid":"...", "email":"..." }}`
* Failures:
  * Invalid current password:

## POST /auth/change_pw

See `PATCH /auth`. This is the exact same functionality.

## POST /auth/update

Allows for the password generation parameters to be updated.

### Headers:

`Authorization: Bearer _JSON_WEB_TOKEN_`

### Params:

All parameters are optional.

#### Optional Params:
```json
{
  "pw_func" : "pbkdf2",
  "pw_alg"  : "sha512",
  "pw_cost" : 5000,
  "pw_key_size": 512,
  "pw_nonce": "", // unused?
  "pw_salt" : "", //sha1 digest of [email, "SN", server's salt value]
  "version" : "002"
}
```

### Responses:
* Success: `200` => `{"token" : "...", "user" : { "uuid":"...", "email":"..." }}`
* Failures: ???
     
# POST /items/sync

Periodically saves/retrieves items stored on the client/server.

### Headers:

`Authorization: Bearer _JSON_WEB_TOKEN_`

### Params:

```json
{
    "sync_token" : "", // Null on first sync
    "cursor_token": "", // Used if possible
    "items" : [
      {
        "uuid": "",
        "content": "",
        "content_type": "",
        "enc_item_key":  "",
        "deleted": false, // optional
        "created_at": "",
        "updated_at": ""  // optional 
      },
      ...
    ]
}
```

#### Optional Params:

```json
{
    "limit" : 100000 // limit the number of items returned from sync.
}
```

### Responses:

* Success: `200` => ` {"retrieved_items" : [], "saved_items" : [], "unsaved_items" : [], "sync_token" : ""}`
* Failures: ???

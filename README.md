# A Rust StandardFile Server Implementation

This is a rust [StandardFile](https://standardfile.org/#api) implementation.

It is a WIP. It was started to learn Iron/Hyper/Tokio and Rust in general. 

It is also giving me an opportunity to mess with GitHub's "Projects" feature.

This will be a mess. Please save me.

### Progress

- Backend Datastore
  - [X] SQLite
  - [ ] MySQL _(TODO to allow drop-in replacement)_
- API v0.0.2
  - [ ] POST /auth
  - [ ] PATCH /auth
  - [ ] POST /auth/sign_in
  - [X] GET /auth/params
  - [ ] POST /items/sync

### Usage:

To test out:

```
$ make run
```


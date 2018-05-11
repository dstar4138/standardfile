# A Rust StandardFile Server Implementation

[![Linux build status](https://travis-ci.org/dstar4138/standardfile.svg?branch=master)](https://travis-ci.org/dstar4138/standardfile)
[![Dependency status](https://deps.rs/repo/github/dstar4138/standardfile/status.svg)](https://deps.rs/repo/github/dstar4138/standardfile)
[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

This is a rust [StandardFile](https://standardfile.org/#api) implementation.

I welcome bug reports and pull requests!

### Usage:

To test out with a simple sqlite db:

```
$ cat .env
export DB_PATH=localite.db
export SALT_PSEUDO_NONCE=123
export SECRET_KEY_BASE=111111111111111
$ source .env && make run
```

To try it out with mysql:

```
$ cat .env
export DB_DATABASE=standardfile
export DB_HOST=localhost
export DB_PORT=3306
export DB_USERNAME=stdfile
export DB_PASSWORD=abc
export SALT_PSEUDO_NONCE=123
export SECRET_KEY_BASE=111111111111111
$ source .env && make run-mysql
```
You could also just run it with docker/docker-compose:

```
$ docker-compose up --build
```

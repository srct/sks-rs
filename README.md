# sks-rs (pronounced "scissors")
[![Build Status](https://travis-ci.org/srct/sks-rs.svg?branch=master)](https://travis-ci.org/srct/sks-rs)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Rust implementation of the Synchronizing Key Server for PGP keys.

We will try to comply with the IETF Draft for [The OpenPGP HTTP Keyserver Protocol (HKP)](https://tools.ietf.org/html/draft-shaw-openpgp-hkp-00).
**********************
DEPRECATION of Gitlab repo:

Effective Jan 1, 2018, ALL references to the gitlab will be phased out, as that repo has been deprecated, largely for better workflow and greater contributor intake here on Github.

**********************

### CHANGELOG

Please see the [CHANGELOG](CHANGELOG.md) for a release history.

### Screenshots

Coming soon!

### Comparison with SKS

Coming soon!

### Why should I use `sks-rs`?

Coming soon!

### Installation

Coming soon!

### Development

It'll be helpful to have [Docker](https://www.docker.com/) installed.

There is a [`docker-compose.yml`](docker-compose.yml) that can be loaded using either `docker stack` or `docker compose`. Run `docker stack deploy -c docker-compose.yml postgres` or `docker-compose up` depending on which tool you prefer to use. This will start [Postgres](https://www.postgresql.org://www.postgresql.org/) on port `5432`, as well as an [Adminer](https://www.adminer.org/) on port `8080`.

Presently (Jan 2018) config.yml and docker-compose.yml use hard-coded user:password credentials, in real world deployment this should use some abstraction layer.  Examples of secure mechanisms for this would be docker_login or some external call to a AS in your infrastructure ((open)ldap,krb5,saml) that would be defined in an auth or environment variable. 

As an example there is a stub file ( `auth_keys_from_cas` ) in this repo that could be used as a file that would have the hashed/encrypted values for tokens/user:pass entries for all auth'd users.

	Example deployment using such a file: 
	* 	User authenticates to local domain/AS server
	*	Server passes (over secure channel communication (DTLS or other secure channel within organization's policies) to this file not much unlike a journal write of a auth attempt for admin/sudo rights. 
	*	Config.yml calls this file to populate auth'd hosts/users and timeout periods of said tokens (if used), to populate it's auth'd user db or update it on some on-demand or periodic schedule.

### Known issues

Coming soon!

### Contributing

Coming soon!

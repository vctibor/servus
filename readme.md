# Servus

Run remote jobs periodically.

Think about this like `cron` with WebUI.


## (Intended) Features:

- Execute commands on remote machines, in scheduled intervals, using `ssh`. ✓

- Log of executed jobs and their exit status. ✓

- Sending emails in case of failure.

- Button to run job immediately.


## Authentication

Application server does not implement any user authentication.

It is intended to be deployed behind `nginx` reverse-proxy with client-side authentication configured and required.

To setup it you can follow [this guide](https://gist.github.com/mtigas/952344).

NOTE: This doesn't seem to be supported at all on Android OS.

## SSH agent

For successful job execution, user under which is `servus` running has to have key-based access to target machine and ssh-agent has to be configured and have imported identity specified by `username` column in `jobs` table entry.

```shell
# start ssh agent
eval `ssh-agent`

# add ssh identities
ssh-add

# list ssh identities
ssh-add -l
```


## Deployment

We use `cargo-deb` to create .deb package (script `build.sh`). This package can be used to install Servus on Debian-derived systems.

You have to set two environment variables:

`SERVUS_DATABASE_URL` should contain connection string to PostgreSQL database.

`SERVUS_LISTEN_ON` should contain IP address and port on which should Servus web interface listen.

Servus is intended to be run as `systemd` service. If that's the case we have to set environment variables in file

`/etc/systemd/system/servus.service.d/local.conf`

like this:

```
[Service]
Environment="SERVUS_DATABASE_URL=postgres://<username>:<password>@<psql_server_ip>/<database_name>"
Environment="SERVUS_LISTEN_ON=<servus_ip>:<servus_port>"
```

https://www.golinuxcloud.com/run-systemd-service-specific-user-group-linux/


## Tech stack

- `Rust` programming language

- `PostgreSQL` database

- `Diesel` ORM and migration tool

- `Actix-web` application server

- `AngularJS` frontend

- `Nginx` reverse proxy


## License

`servus` is licensed under terms of GPLv3 license.
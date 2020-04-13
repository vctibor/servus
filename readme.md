# Servus

Run remote jobs periodically.

Think about this like `cron` with WebUI.


## (Intended) Features:

- Execute commands on remote machines, in scheduled intervals, using `ssh`.

- Log of executed jobs and their exit status.

- Sending emails in case of failure.

- Button to run job immediately.


## Authentication

Application server does not implement any user authentication.

It is intended to be deployed behind `nginx` reverse-proxy with client-side authentication configured and required.

To setup it you can follow [this guide](https://gist.github.com/mtigas/952344).


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

You have to set two environment variables:




## Tech stack

- `Rust` programming language

- `PostgreSQL` database

- `Diesel` ORM and migration tool

- `Actix-web` application server

- `AngularJS` frontend

- `Nginx` reverse proxy


## License

`servus` is licensed under terms of GPLv3 license.
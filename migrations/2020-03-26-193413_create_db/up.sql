/*
Dependencies:

tx_log ---> jobs
               |---> machines
               |---> users
*/

-- Represents owner of task. Mainly for communication (email, etc.)
--  in case of failed task.
create table users (
    id uuid primary key,
    name text not null,
    email text
);

-- Represents login to remote machine. We expect passwordless login.
create table machines (
    id uuid primary key,
    name text not null,     -- Friendly alias of the machine.
    username text not null, -- Login user.
    url text not null,
    port integer not null
);

-- Represents task to be executed periodically.
create table jobs (
    id uuid primary key,
    name text not null,     -- Friendly alias.
    code text not null,     -- Platform specific shell code to be executed.
    description text,       -- Optional task desription.
    schedule text not null, -- Schedule in cron syntax.
    target uuid not null references machines(id),   -- Pointer into 'machines' table.
    owner uuid not null references users(id),
    last_update timestamp,  -- Changes whenever any other column is changed. All scheduled instances are dropped and rescheduled.
    send_email boolean not null      -- If true sends email to owner in case of failed execution.
);

-- Represents log of executed tasks, with their exit status.
create table tx_log (
    id uuid primary key,
    stdout text,
    stderr text,
    success boolean not null,
    time timestamp not null,
    message text not null,
    job uuid not null       -- No FK so we can retain logs realted to deleted jobs.
);
# Servus

Task execution server.

Think about this basically as a *cron* with WebUI.

## Features:

- execute commands on remote machines, in scheduled intervals, using ssh

- log of executed tasks and their exist status

- sending emails in case of failure

## Database

### Tables

**job:**

Represent scheduled task: command to be executed on remote machine periodically.

Name        | Type       | Null |Description
------------|------------|------|-------------
job_id      | UUID       | no   |
name        | Text       | no   |
code        | Text       | no   | Shell source code for task to be executed.
description | Text       | yes  | Allows to create text to describe this task.
schedule    | Text       | no   |
target      | FK machine | no   |
owner       | FK user    | no   |
last_update | data       | no   | Has to be updated whenever theres change in other columns. All scheduled instances will be dropped and job will be rescheduled.

**target:**

Represents remote machines on which we execute our commands. Login credentials.

Name       | Type       | Null |Description
-----------|------------|------|-------------
target_ID  | UUID       | no   |
Name       | Text       | no   |
Username   | Text       | no   |
URL        | Text       | no   |
Port       | number     | no   |

**user:**

Represents owner of task: person with email address to be notified if task fails to execute.

Name       | Type       | Null |Description
-----------|------------|------|-------------
owner_ID   | UUID       | no   |
Name       | Text       | no   |
Email      | Text       | yes  |

**tx_log:**

Transactions log of performed tasks.

Name        | Type       | Null |Description
------------|------------|------|-------------
tx_log_ID   | UUID       | no   |
Success     | bool       | no   |
Time        | date       | no   |
Description | text       | no   |

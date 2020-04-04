use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::entity::Machine as MachineEntity;
use crate::schema::machines;
use crate::schema::machines::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Identifiable, Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
pub struct Machine {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub url: String,
    pub port: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewMachine {
    pub name: String,
    pub username: String,
    pub url: String,
    pub port: i32
}

pub fn add_machine(machine: MachineEntity, conn: &PgConnection)
                -> Result<MachineEntity, diesel::result::Error>
{
    let machine = Machine {
        id: Uuid::new_v4(),
        name: machine.name,
        username: machine.username,
        url: machine.url,
        port: machine.port
    };

    diesel::insert_into(machines)
        .values(&machine)
        .execute(conn)?;

    Ok(MachineEntity {
        id: Some(machine.id),
        name: machine.name,
        username: machine.username,
        url: machine.url,
        port: machine.port
    })
}

pub fn get_machines(conn: &PgConnection)
                 -> Result<Vec<MachineEntity>, diesel::result::Error>
{
    let machine_table: Vec<Machine> = machines.load::<Machine>(conn)?;

    let mut entities = Vec::with_capacity(machine_table.len());

    for machine in machine_table.iter() {
        entities.push(MachineEntity {
            id: Some(machine.id),
            name: machine.name.to_owned(),
            username: machine.username.to_owned(),
            url: machine.url.to_owned(),
            port: machine.port
        });
    }
    
    Ok(entities)
}

pub fn get_machine(uid: Uuid, conn: &PgConnection)
                -> Result<Option<MachineEntity>, diesel::result::Error>
{
    let machine = machines
        .filter(id.eq(uid))
        .first::<Machine>(conn)
        .optional()?;

    match machine {
        Some(m) => Ok(Some(MachineEntity {
            id: Some(m.id),
            name: m.name,
            username: m.username,
            url: m.url,
            port: m.port
        })),
        None => Ok(None),
    }
}

pub fn update_machine(machine: MachineEntity, machine_id: Uuid, conn: &PgConnection)
                   -> Result<usize, diesel::result::Error>
{
    let machine = Machine {
        id: machine_id,
        name: machine.name,
        username: machine.username,
        url: machine.url,
        port: machine.port
    };

    diesel::update(machines::table).set(&machine).execute(conn)
}

pub fn delete_machine(uid: Uuid, conn: &PgConnection)
                   -> Result<usize, diesel::result::Error>
{
    diesel::delete(machines.filter(id.eq(uid))).execute(conn)
}
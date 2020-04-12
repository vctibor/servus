use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::entity::AnyError;
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
    let machine_table: Vec<Machine> = machines.order(name).load::<Machine>(conn)?;

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

    diesel::update(machines::table).filter(id.eq(machine_id)).set(&machine).execute(conn)
}

pub fn update_machines(mut updated_machines: Vec<MachineEntity>, conn: &PgConnection)
                  -> Result<(), AnyError>
{    
    let old_machines: Vec<MachineEntity> = get_machines(&conn)?;
    let old_machines_ids: Vec<Uuid> = old_machines.into_iter().map(|machine| machine.id.unwrap()).rev().collect();

    let mut machines_to_delete = old_machines_ids.clone();

    while let Some(updated_machine) = updated_machines.pop() {
        if updated_machine.id.is_none() || updated_machine.id == Some(Uuid::nil())
        {
            add_machine(updated_machine, &conn)?;
        }
        else if let Some(updated_machine_id) = updated_machine.id
        {
            if old_machines_ids.contains(&updated_machine_id) {
                update_machine(updated_machine, updated_machine_id, &conn)?;
            } else {
                add_machine(updated_machine, &conn)?;
            }

            machines_to_delete.retain(|&item| item != updated_machine_id);
        }
    }

    for delete_id in machines_to_delete {
        delete_machine(delete_id, &conn)?;
    }

    Ok(())
}

pub fn delete_machine(uid: Uuid, conn: &PgConnection)
                   -> Result<usize, diesel::result::Error>
{
    diesel::delete(machines.filter(id.eq(uid))).execute(conn)
}
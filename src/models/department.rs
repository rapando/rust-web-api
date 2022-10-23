use log::{info, warn};
use mysql::prelude::Queryable;
use mysql::{params, PooledConn};
use serde::Serialize;

#[derive(Serialize)]
pub struct Department {
    pub id: u64,
    pub name: String,
    pub staff_count: u16,
    pub created: String,
    pub modified: String,
}

pub fn create(
    name: &String,
    staff_count: u16,
    conn: &mut PooledConn,
) -> Result<u64, mysql::error::Error> {
    let query = "INSERT INTO department (name, staff_count) \
    VALUES (:name,:staff_count)";
    match conn
        .exec_drop(
            query,
            params! {
                "name" => name,
                "staff_count" => staff_count,
            },
        )
        .and_then(|_| Ok(conn.last_insert_id()))
    {
        Ok(id) => {
            info!("inserted department {} with id {}", name, id);
            Ok(id)
        }
        Err(e) => {
            warn!("failed to create department {} because {:?}", name, e);
            Err(e)
        }
    }
}

pub fn get_all(conn: &mut PooledConn) -> Result<Vec<Department>, mysql::error::Error> {
    let query = "SELECT id, name, staff_count, created, modified \
    FROM department";
    match conn.query_map(query, |(id, name, staff_count, created, modified)| {
        Department {
            id,
            name,
            staff_count,
            created,
            modified,
        }
    }) {
        Ok(rows) => Ok(rows),
        Err(e) => {
            warn!("unable to fetch departments because : {:?}", e);
            Err(e)
        }
    }
}

pub fn get_one(id: u64, conn: &mut PooledConn) -> Option<Department> {
    let query = format!(
        "SELECT id, name, staff_count, created, modified \
        FROM department \
        WHERE id='{}' \
        LIMIT 1",
        id
    );
    match conn.query_map(query, |(id, name, staff_count, created, modified)| {
        Department {
            id,
            name,
            staff_count,
            created,
            modified,
        }
    }) {
        Ok(mut rows) => rows.pop(),
        Err(e) => {
            warn!("unable to fetch departments because : {:?}", e);
            None
        }
    }
}

pub fn update(
    id: u64,
    name: &String,
    staff_count: u16,
    conn: &mut PooledConn,
) -> Result<bool, mysql::error::Error> {
    let query = "UPDATE department \
    SET name=:name, staff_count=:staff_count \
    WHERE id=:id \
    LIMIT 1";
    match conn
        .exec_drop(
            query,
            params! {
                "name" => name,
                "staff_count" => staff_count,
                "id" => id,
            },
        )
        .and_then(|_| Ok(conn.affected_rows()))
    {
        Ok(_) => Ok(true),
        Err(e) => {
            warn!("failed to update department {} because {:?}", id, e);
            Err(e)
        }
    }
}

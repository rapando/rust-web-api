use log::{info, warn};
use mysql::{params, PooledConn};
use mysql::prelude::Queryable;
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub department_id: u64,
    pub department_name: String,
    pub created: String,
    pub modified: String,
}

pub fn create(first_name: &String, last_name: &String, department_id: u64, salt: &String, password: &String, conn: &mut PooledConn) -> Result<u64, mysql::error::Error> {
    let query = "INSERT INTO user (first_name, last_name, department_id, salt, password) \
    VALUES (:first_name, :last_name, :department_id, :salt, :password)";
    match conn.exec_drop(query, params! {
        "first_name" => first_name,
        "last_name" => last_name,
        "department_id" => department_id,
        "salt" => salt,
        "password" => password,
    },
    ).and_then(|_| Ok(conn.last_insert_id())) {
        Ok(id) => {
            info!("inserted user {} with id {}", first_name, id);
            Ok(id)
        }
        Err(e) => {
            warn!("failed to add user {} because {:?}", first_name, e);
            Err(e)
        }
    }
}

pub fn get_all(conn: &mut PooledConn) -> Result<Vec<User>, mysql::error::Error> {
    let query = "SELECT u.id, u.first_name, u.last_name, u.department_id, \
    d.name as department_name, u.created, u.modified \
    FROM user u \
    INNER JOIN department d ON d.id = u.department_id";
    match conn.query_map(query,
                         |(id, first_name, last_name, department_id, department_name, created, modified)| User {
                             id,
                             first_name,
                             last_name,
                             department_id,
                             department_name,
                             created,
                             modified,
                         },
    ) {
        Ok(rows) => {
            Ok(rows)
        }
        Err(e) => {
            warn!("unable to fetch users because : {:?}", e);
            Err(e)
        }
    }
}

pub fn get_one(id:u64, conn: &mut PooledConn) -> Option<User> {
    let query = format!("SELECT u.id, u.first_name, u.last_name, u.department_id, \
    d.name as department_name, u.created, u.modified \
    FROM user u \
    INNER JOIN department d ON d.id = u.department_id \
    WHERE u.user_id='{}' \
    LIMIT 1", id);
    match conn.query_map(query,
                         |(id, first_name, last_name, department_id, department_name, created, modified)| User {
                             id,
                             first_name,
                             last_name,
                             department_id,
                             department_name,
                             created,
                             modified,
                         },
    ) {
        Ok(mut rows) => {
            rows.pop()
        }
        Err(e) => {
            warn!("unable to fetch users because : {:?}", e);
            None
        }
    }
}

pub fn update(id: u64, first_name: &String, last_name: &String, department_id: u64, conn: &mut PooledConn) -> Result<bool, mysql::error::Error> {
    let query = "UPDATE user \
    SET first_name=:first_name, last_name=:last_name, department_id=:department_id \
    WHERE id=:id \
    LIMIT 1";
    match conn.exec_drop(query, params! {
        "first_name" => first_name,
        "last_name" => last_name,
        "department_id" => department_id,
        "id" => id,
    },
    ).and_then(|_| Ok(conn.affected_rows())) {
        Ok(_) => {
            Ok(true)
        }
        Err(e) => {
            warn!("failed to update user {} because {:?}", first_name, e);
            Err(e)
        }
    }
}
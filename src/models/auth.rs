use mysql::PooledConn;

struct StoredAuthData {
    salt: String,
    password: String,
}

pub fn login(user_name: &String, password: &String, conn: &mut PooledConn) {
    let query = "SELECT salt, password FROM user WHERE user_name=? LIMIT 1";
    // let stored_auth_data = match conn.query_map
    
}

pub fn reset_password(
    user_name: &String,
    password: &String,
    new_password: &String,
    conn: &mut PooledConn,
) {
}

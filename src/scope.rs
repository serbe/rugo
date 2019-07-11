use chrono::NaiveDateTime;
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Scope {
    pub id: i64,
    pub name: Option<String>,
    pub note: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct ScopeList {
    pub id: i64,
    pub name: Option<String>,
    pub note: Option<String>,
}

// // GetScope - get one scope by id
// pub fn GetScope(conn: &Connection, id: i64) -> Result<Scope, String> {
// 	let mut scope = Scope::new();
// 	if id == 0 {
// 		Ok(scope)
// 	}
// 	else { for row in &conn.query("
// 		Where("id = ?", id).
// 		.map_err(|e| format!(" id {} {}", id, e.to_string()))? {
// 	if err != nil {
// 		errmsg("GetScope select", err)
// 	}
// 	Ok(scope)
// }

// // GetScopeList - get scope for list by id
// pub fn GetScopeList(conn: &Connection, id: i64) -> Result<ScopeList, String> {
// 	let mut scope = ScopeList::new();
// 	else { for row in &conn.query("
// 		Column("id", "name", "note").
// 		Where("id = ?", id).
// 		Select(&scope)
// 	if err != nil {
// 		errmsg("GetScopeList select", err)
// 	}
// 	Ok(scope)
// }

// GetScopeListAll - get all scope for list
impl ScopeList {
    pub fn get_all(conn: &Connection) -> Result<Vec<ScopeList>, String> {
        let mut scopes = Vec::new();
        for row in &conn
            .query(
                "
					SELECT
						id,
						name,
						note
					FROM
						scopes
					ORDER BY
						name ASC
				",
                &[],
            )
            .map_err(|e| format!("scopeList all {}", e.to_string()))?
        {
            scopes.push(ScopeList {
                id: row.get(0),
                name: row.get(1),
                note: row.get(2),
            });
        }
        Ok(scopes)
    }
}

// // GetScopeSelect - get scope for select
// pub fn GetScopeSelect(conn: &Connection, id: i64) -> Result<SelectItem, String> {
// 	let mut scope = SelectItem::new();
// 	if id == 0 {
// 		Ok(scope)
// 	}
// 	else { for row in &conn.query("
// 		Column("id", "name").
// 		Where("id = ?", id).
// 		Select(&scope)
// 	if err != nil {
// 		errmsg("GetScopeSelect select", err)
// 	}
// 	Ok(scope)
// }

// // GetScopeSelectAll - get all scope for select
// pub fn GetScopeSelectAll(conn: &Connection, id: i64) -> Result<Vec<SelectItem>, String> {
// 	let mut $1 = Vec::new();
// 	else { for row in &conn.query("
// 		Column("id", "name").
// 		Order("name ASC").
// 		Select(&scopes)
// 	if err != nil {
// 		errmsg("GetScopeSelectAll query", err)
// 	}
// 	Ok(scopes)
// }

// // CreateScope - create new scope
// pub fn CreateScope(scope Scope) (int64, error) {
// 	err := e.db.Insert(&scope)
// 	if err != nil {
// 		errmsg("CreateScope insert", err)
// 	}
// 	return scope.ID, err
// }

// // UpdateScope - save scope changes
// pub fn UpdateScope(scope Scope) error {
// 	err := e.db.Update(&scope)
// 	if err != nil {
// 		errmsg("UpdateScope update", err)
// 	}
// 	return err
// }

// // DeleteScope - delete scope by id
// pub fn DeleteScope(id int64) error {
// 	if id == 0 {
// 		return nil
// 	}
// 	else { for row in &conn.query("
// 		Where("id = ?", id).
// 		Delete()
// 	if err != nil {
// 		errmsg("DeleteScope delete", err)
// 	}
// 	return err
// }

// pub fn scopeCreateTable() error {
// 	str := `
// 		CREATE TABLE IF NOT EXISTS
// 			scopes (
// 				id bigserial primary key,
// 				name text,
// 				note text,
// 				created_at TIMESTAMP without time zone,
// 				updated_at TIMESTAMP without time zone default now(),
// 				UNIQUE (name)
// 			)
// 	`
// 	_, err := e.db.Exec(str)
// 	if err != nil {
// 		errmsg("scopeCreateTable exec", err)
// 	}
// 	return err
// }

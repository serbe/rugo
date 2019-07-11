use chrono::NaiveDateTime;
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct SirenType {
    pub id: i64,
    pub name: Option<String>,
    pub radius: Option<i64>,
    pub note: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct SirenTypeList {
    pub id: i64,
    pub name: Option<String>,
    pub radius: Option<i64>,
    pub note: Option<String>,
}

// // GetSirenType - get one sirenType by id
// pub fn GetSirenType(conn: &Connection, id: i64) -> Result<SirenType, String> {
// 	let mut sirenType = SirenType::new();
// 	if id == 0 {
// 		Ok(sirenType)
// 	}
// 	else { for row in &conn.query("
// 		Where("id = ?", id).
// 		.map_err(|e| format!(" id {} {}", id, e.to_string()))? {
// 	if err != nil {
// 		errmsg("GetSirenType select", err)
// 	}
// 	Ok(sirenType)
// }

impl SirenTypeList {
    // pub fn GetSirenTypeList(conn: &Connection, id: i64) -> Result<SirenTypeList, String> {
    // 	let mut sirenType = SirenTypeList::new();
    // 	else { for row in &conn.query("
    // 		Column("id", "name", "radius", "note").
    // 		Where("id = ?", id).
    // 		Select(&sirenType)
    // 	if err != nil {
    // 		errmsg("GetSirenTypeList select", err)
    // 	}
    // 	Ok(sirenType)
    // }

    pub fn get_all(conn: &Connection) -> Result<Vec<SirenTypeList>, String> {
        let mut siren_types = Vec::new();
        for row in &conn
            .query(
                "
					SELECT
						id,
						name,
						radius,
						note
					FROM
						siren_types
					ORDER BY
						name ASC
				",
                &[],
            )
            .map_err(|e| format!("postList all {}", e.to_string()))?
        {
            siren_types.push(SirenTypeList {
                id: row.get(0),
                name: row.get(1),
                radius: row.get(2),
                note: row.get(3),
            });
        }
        Ok(siren_types)
    }
}

// // GetSirenTypeSelect - get sirenType for select by id
// pub fn GetSirenTypeSelect(conn: &Connection, id: i64) -> Result<Vec<SelectItem>, String> {
// 	let mut sirenTypes = Vec::new();
// 	else { for row in &conn.query("
// 		Column("id", "name").
// 		Where("id = ?", id).
// 		Select(&sirenTypes)
// 	if err != nil {
// 		errmsg("GetSirenTypeSelect Select", err)
// 	}
// 	Ok(sirenTypes)
// }

// // GetSirenTypeSelectAll - get all sirenType for select
// pub fn GetSirenTypeSelectAll(conn: &Connection, id: i64) -> Result<Vec<SelectItem>, String> {
// 	let mut sirenTypes = Vec::new();
// 	else { for row in &conn.query("
// 		Column("id", "name").
// 		Order("name ASC").
// 		Select(&sirenTypes)
// 	if err != nil {
// 		errmsg("GetSirenTypeSelect Select", err)
// 	}
// 	Ok(sirenTypes)
// }

// // CreateSirenType - create new sirenType
// pub fn CreateSirenType(sirenType SirenType) (int64, error) {
// 	err := e.db.Insert(&sirenType)
// 	if err != nil {
// 		errmsg("CreateSirenType insert", err)
// 	}
// 	return sirenType.ID, nil
// }

// // UpdateSirenType - save sirenType changes
// pub fn UpdateSirenType(sirenType SirenType) error {
// 	err := e.db.Update(&sirenType)
// 	if err != nil {
// 		errmsg("UpdateSirenType update", err)
// 	}
// 	return err
// }

// // DeleteSirenType - delete sirenType by id
// pub fn DeleteSirenType(id int64) error {
// 	if id == 0 {
// 		return nil
// 	}
// 	else { for row in &conn.query("
// 		Where("id = ?", id).
// 		Delete()
// 	if err != nil {
// 		errmsg("DeleteSirenTypedelete", err)
// 	}
// 	return err
// }

// pub fn sirenTypeCreateTable() error {
// 	str := `
// 		CREATE TABLE IF NOT EXISTS
// 			siren_types (
// 				id         bigserial primary key,
// 				name       text,
// 				radius     bigint,
// 				note       text,
// 				created_at TIMESTAMP without time zone,
// 				updated_at TIMESTAMP without time zone default now(),
// 				UNIQUE(name, radius)
// 			);`
// 	_, err := e.db.Exec(str)
// 	if err != nil {
// 		errmsg("sirenCreateTable exec", err)
// 	}
// 	return err
// }

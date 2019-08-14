use chrono::{Local, NaiveDateTime};
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SirenType {
    pub id: i64,
    pub name: Option<String>,
    pub radius: Option<i64>,
    pub note: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SirenTypeList {
    pub id: i64,
    pub name: Option<String>,
    pub radius: Option<i64>,
    pub note: Option<String>,
}

impl SirenType {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(conn: &Connection, id: i64) -> Result<SirenType, String> {
        let mut siren_type = SirenType::new();
        if id == 0 {
            Ok(siren_type)
        } else {
            for row in &conn
                .query(
                    "
                        SELECT
                            name,
                            radius,
                            note,
                            created_at,
                            updated_at
                        FROM
                            siren_types
                        WHERE
                            id = $1
                    ",
                    &[&id],
                )
                .map_err(|e| format!("syren_type id {} {}", id, e.to_string()))?
            {
                siren_type = SirenType {
                    id,
                    name: row.get(0),
                    radius: row.get(1),
                    note: row.get(2),
                    created_at: row.get(3),
                    updated_at: row.get(4),
                };
            }
            Ok(siren_type)
        }
    }

    pub fn post(conn: &Connection, id: i64, siren_type: SirenType) -> Result<SirenType, String> {
        if id == 0 {
            SirenType::insert(conn, siren_type)
        } else {
            SirenType::update(conn, id, siren_type)
        }
    }

    pub fn insert(conn: &Connection, siren_type: SirenType) -> Result<SirenType, String> {
        let mut siren_type = siren_type;
        for row in &conn
            .query(
                "
                    INSERT INTO siren_types
                    (
                        name,
                        radius,
                        note,
                        created_at,
                        updated_at
                    )
                    VALUES
                    (
                        $1,
                        $2,
                        $3,
                        $4,
                        $5
                    )
                    RETURNING
                        id
                ",
                &[
                    &siren_type.name,
                    &siren_type.radius,
                    &siren_type.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .map_err(|e| format!("create siren_type {} ", e.to_string()))?
        {
            siren_type.id = row.get(0)
        }
        Ok(siren_type)
    }

    pub fn update(conn: &Connection, id: i64, siren_type: SirenType) -> Result<SirenType, String> {
        let mut siren_type = siren_type;
        siren_type.id = id;
        match &conn.execute(
            "
                UPDATE siren_types SET
                    name = $2,
                    radius = $3,
                    note = $4,
                    updated_at = $5
                WHERE
                    id = $1
            ",
            &[
                &siren_type.id,
                &siren_type.name,
                &siren_type.radius,
                &siren_type.note,
                &Local::now().naive_local(),
            ],
        ) {
            Ok(0) => Err(format!("update siren_type id {}", id)),
            _ => Ok(siren_type),
        }
    }
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

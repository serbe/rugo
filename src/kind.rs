use chrono::{Local, NaiveDateTime};
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Kind {
    pub id: i64,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KindList {
    pub id: i64,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub note: Option<String>,
}

impl Kind {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(conn: &Connection, id: i64) -> Result<Kind, String> {
        let mut kind = Kind::new();
        if id == 0 {
            Ok(kind)
        } else {
            for row in &conn
                .query(
                    "
						SELECT
							name,
							short_name,
							note,
							created_at,
							updated_at
						FROM
							kinds
						WHERE
							id = $1
					",
                    &[&id],
                )
                .map_err(|e| format!("kind id {} {}", id, e.to_string()))?
            {
                kind = Kind {
                    id,
                    name: row.get(0),
                    short_name: row.get(1),
                    note: row.get(2),
                    created_at: row.get(3),
                    updated_at: row.get(4),
                }
            }
            Ok(kind)
        }
    }

    pub fn post(conn: &Connection, id: i64, kind: Kind) -> Result<Kind, String> {
        if id == 0 {
            Kind::insert(conn, kind)
        } else {
            Kind::update(conn, id, kind)
        }
    }

    pub fn insert(conn: &Connection, kind: Kind) -> Result<Kind, String> {
        let mut kind = kind;
        for row in &conn
            .query(
                "
                    INSERT INTO kinds
                    (
                        name,
                        short_name,
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
                    &kind.name,
                    &kind.note,
                    &kind.short_name,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .map_err(|e| format!("create kind {} ", e.to_string()))?
        {
            kind.id = row.get(0)
        }
        Ok(kind)
    }

    pub fn update(conn: &Connection, id: i64, kind: Kind) -> Result<Kind, String> {
        let mut kind = kind;
        kind.id = id;
        match &conn.execute(
            "
                UPDATE kinds SET
                    name = $2,
                    short_name = $3,
                    note = $4,
                    updated_at = $5
                WHERE
                    id = $1
            ",
            &[
                &kind.id,
                &kind.name,
                &kind.short_name,
                &kind.note,
                &Local::now().naive_local(),
            ],
        ) {
            Ok(0) => Err(format!("update kind id {}", id)),
            _ => Ok(kind),
        }
    }
}

impl KindList {
    // pub fn new() -> Self {
    // 	Default::default()
    // }

    // pub fn get(conn: &Connection, id: i64) -> Result<KindList, String> {
    // 	let mut kind = KindList::new();
    // 	if id == 0 {
    // 		Ok(kind)
    // 	} else {
    // 		for row in &conn
    // 			.query(
    // 				"
    // 					SELECT
    // 						name,
    // 						short_name,
    // 						note
    // 					FROM
    // 						kinds
    // 					WHERE
    // 						id = $1
    // 				",
    // 				&[&id],
    // 			)
    // 			.map_err(|e| format!("kind list id {} {}", id, e.to_string()))?
    // 		{
    // 			kind = KindList {
    // 				id,
    // 				name: row.get(0),
    // 				short_name: row.get(1),
    // 				note: row.get(2),
    // 			}
    // 		}
    // 		Ok(kind)
    // 	}
    // }

    pub fn get_all(conn: &Connection) -> Result<Vec<KindList>, String> {
        let mut kinds = Vec::new();
        for row in &conn
            .query(
                "
					SELECT
						id,
						name,
						short_name,
						note
					FROM
						kinds
					ORDER BY
						name ASC
				",
                &[],
            )
            .map_err(|e| format!("kind list all {}", e.to_string()))?
        {
            kinds.push(KindList {
                id: row.get(0),
                name: row.get(1),
                short_name: row.get(2),
                note: row.get(3),
            });
        }
        Ok(kinds)
    }
}

// // CreateKind - create new kind
// pub fn CreateKind(kind Kind) (int64, error) {
// 	err := e.db.Insert(&kind)
// 	if err != nil {
// 		errmsg("CreateKind insert", err)
// 	}
// 	return kind.ID, nil
// }

// // UpdateKind - save kind changes
// pub fn UpdateKind(kind Kind) error {
// 	err := e.db.Update(&kind)
// 	if err != nil {
// 		errmsg("UpdateKind update", err)
// 	}
// 	return err
// }

// // DeleteKind - delete kind by id
// pub fn DeleteKind(id int64) error {
// 	if id == 0 {
// 		return nil
// 	}
// 	else { for row in &conn.query("
// 		Where("id = ?", id).
// 		Delete()
// 	if err != nil {
// 		errmsg("DeleteKind delete", err)
// 	}
// 	return err
// }

// pub fn kindCreateTable() error {
// 	str := `
// 		CREATE TABLE IF NOT EXISTS
// 			kinds (
// 				id bigserial primary key,
// 				name text,
// 				short_name text,
// 				note text,
// 				created_at TIMESTAMP without time zone,
// 				updated_at TIMESTAMP without time zone default now(),
// 				UNIQUE(name)
// 			)
// 	`
// 	_, err := e.db.Exec(str)
// 	if err != nil {
// 		errmsg("kindCreateTable exec", err)
// 	}
// 	return err
// }

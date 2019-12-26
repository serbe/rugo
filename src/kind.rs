use chrono::{Local, NaiveDateTime};
use postgres::Client;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Kind {
    #[serde(default)]
    pub id: i64,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize)]
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

    pub fn get(conn: &mut Client, id: i64) -> Result<Kind, String> {
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

    pub fn insert(conn: &mut Client, kind: Kind) -> Result<Kind, String> {
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

    pub fn update(conn: &mut Client, kind: Kind) -> Result<Kind, String> {
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
            Ok(0) => Err(format!("update kind id {}", kind.id)),
            _ => Ok(kind),
        }
    }

    pub fn delete(conn: &mut Client, id: i64) -> bool {
        conn.execute(
            "
                DELETE FROM
                    kinds
                WHERE
                    id = $1
            ",
            &[&id],
        )
        .is_ok()
    }
}

impl KindList {
    pub fn get_all(conn: &mut Client) -> Result<Vec<KindList>, String> {
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

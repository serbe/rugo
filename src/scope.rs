use chrono::{Local, NaiveDateTime};
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Scope {
    #[serde(default)]
    pub id: i64,
    pub name: Option<String>,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct ScopeList {
    pub id: i64,
    pub name: Option<String>,
    pub note: Option<String>,
}

impl Scope {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(conn: &Connection, id: i64) -> Result<Scope, String> {
        let mut scope = Scope::new();
        if id == 0 {
            Ok(scope)
        } else {
            for row in &conn
                .query(
                    "
                        SELECT
                            name,
                            note,
                            created_at,
                            updated_at
                        FROM
                            scopes
                        WHERE
                            id = $1
                    ",
                    &[&id],
                )
                .map_err(|e| format!("scope id {} {}", id, e.to_string()))?
            {
                scope = Scope {
                    id,
                    name: row.get(0),
                    note: row.get(1),
                    created_at: row.get(2),
                    updated_at: row.get(3),
                };
            }
            Ok(scope)
        }
    }

    pub fn insert(conn: &Connection, scope: Scope) -> Result<Scope, String> {
        let mut scope = scope;
        for row in &conn
            .query(
                "
                    INSERT INTO scopes
                    (
                        name,
                        note,
                        created_at,
                        updated_at
                    )
                    VALUES
                    (
                        $1,
                        $2,
                        $3,
                        $4
                    )
                    RETURNING
                        id
                ",
                &[
                    &scope.name,
                    &scope.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .map_err(|e| format!("create scope {} ", e.to_string()))?
        {
            scope.id = row.get(0)
        }
        Ok(scope)
    }

    pub fn update(conn: &Connection, scope: Scope) -> Result<Scope, String> {
        match &conn.execute(
            "
                UPDATE scopes SET
                    name = $2,
                    note = $3,
                    updated_at = $4
                WHERE
                    id = $1
            ",
            &[
                &scope.id,
                &scope.name,
                &scope.note,
                &Local::now().naive_local(),
            ],
        ) {
            Ok(0) => Err(format!("update scope id {}", scope.id)),
            _ => Ok(scope),
        }
    }

    pub fn delete(conn: &Connection, id: i64) -> bool {
        conn.execute(
            "
                DELETE FROM
                    scopes
                WHERE
                    id = $1
            ",
            &[&id],
        )
        .is_ok()
    }
}

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

use chrono::{Local, NaiveDateTime};
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Department {
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
pub struct DepartmentList {
    pub id: i64,
    pub name: Option<String>,
    pub note: Option<String>,
}

impl Department {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(conn: &Connection, id: i64) -> Result<Department, String> {
        let mut department = Department::new();
        if id == 0 {
            Ok(department)
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
							departments
						WHERE
							id = $1
					",
                    &[&id],
                )
                .map_err(|e| format!("department id {} {}", id, e.to_string()))?
            {
                department = Department {
                    id,
                    name: row.get(0),
                    note: row.get(1),
                    created_at: row.get(2),
                    updated_at: row.get(3),
                }
            }
            Ok(department)
        }
    }

    pub fn post(conn: &Connection, id: i64, department: Department) -> Result<Department, String> {
        if id == 0 {
            Department::insert(conn, department)
        } else {
            Department::update(conn, id, department)
        }
    }

    pub fn insert(conn: &Connection, department: Department) -> Result<Department, String> {
        let mut department = department;
        for row in &conn
            .query(
                "
                    INSERT INTO departments
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
                    &department.name,
                    &department.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .map_err(|e| format!("create department {} ", e.to_string()))?
        {
            department.id = row.get(0)
        }
        Ok(department)
    }

    pub fn update(
        conn: &Connection,
        id: i64,
        department: Department,
    ) -> Result<Department, String> {
        let mut department = department;
        department.id = id;
        match &conn.execute(
            "
                UPDATE departments SET
                    name = $2,
                    note = $3,
                    updated_at = $4
                WHERE
                    id = $1
            ",
            &[
                &department.id,
                &department.name,
                &department.note,
                &Local::now().naive_local(),
            ],
        ) {
            Ok(0) => Err(format!("update department id {}", id)),
            _ => Ok(department),
        }
    }

    pub fn delete(conn: &Connection, id: i64) -> bool {
        conn.execute(
            "
                DELETE FROM
                    departments
                WHERE
                    id = $1
            ",
            &[&id],
        )
        .is_ok()
    }
}

impl DepartmentList {
    pub fn get_all(conn: &Connection) -> Result<Vec<DepartmentList>, String> {
        let mut departments = Vec::new();
        for row in &conn
            .query(
                "
					SELECT
						id,
						name,
						note
					FROM
						departments
					ORDER BY
						name ASC
				",
                &[],
            )
            .map_err(|e| format!("department list all {}", e.to_string()))?
        {
            departments.push(DepartmentList {
                id: row.get(0),
                name: row.get(1),
                note: row.get(2),
            });
        }
        Ok(departments)
    }
}

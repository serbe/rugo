use chrono::{Local, NaiveDateTime};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};

use anyhow::Result;

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

#[derive(Deserialize, Serialize)]
pub struct DepartmentList {
    pub id: i64,
    pub name: Option<String>,
    pub note: Option<String>,
}

impl Department {
    // pub fn new() -> Self {
    //     Default::default()
    // }

    pub async fn get(client: &Client, id: i64) -> Result<Department> {
        let stmt = client
            .prepare(
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
            )
            .await?;
        let row = client.query_one(&stmt, &[&id]).await?;
        let department = Department {
            id,
            name: row.try_get(0)?,
            note: row.try_get(1)?,
            created_at: row.try_get(2)?,
            updated_at: row.try_get(3)?,
        };
        Ok(department)
    }

    pub async fn insert(client: &Client, department: Department) -> Result<Department> {
        let mut department = department;
        let stmt = client
            .prepare(
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
            )
            .await?;
        let row = client
            .query_one(
                &stmt,
                &[
                    &department.name,
                    &department.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .await?;
        department.id = row.get(0);
        Ok(department)
    }

    pub async fn update(client: &Client, department: Department) -> Result<u64> {
        let stmt = client
            .prepare(
                "
                    UPDATE departments SET
                        name = $2,
                        note = $3,
                        updated_at = $4
                    WHERE
                        id = $1
                    ",
            )
            .await?;
        Ok(client
            .execute(
                &stmt,
                &[
                    &department.id,
                    &department.name,
                    &department.note,
                    &Local::now().naive_local(),
                ],
            )
            .await?)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<u64> {
        let stmt = client
            .prepare(
                "
                    DELETE FROM
                        departments
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client.execute(&stmt, &[&id]).await?)
    }
}

impl DepartmentList {
    pub async fn get_all(client: &Client) -> Result<Vec<DepartmentList>> {
        let mut departments = Vec::new();
        let stmt = client
            .prepare(
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
            )
            .await?;
        for row in client.query(&stmt, &[]).await? {
            departments.push(DepartmentList {
                id: row.try_get(0)?,
                name: row.try_get(1)?,
                note: row.try_get(2)?,
            });
        }
        Ok(departments)
    }
}

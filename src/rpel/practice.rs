use chrono::{Local, NaiveDate, NaiveDateTime};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};

use crate::error::ServiceError;

#[derive(Default, Deserialize, Serialize)]
pub struct Practice {
    #[serde(default)]
    pub id: i64,
    pub company_id: Option<i64>,
    pub kind_id: Option<i64>,
    pub topic: Option<String>,
    pub date_of_practice: Option<NaiveDate>,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize)]
pub struct PracticeList {
    pub id: i64,
    pub company_id: Option<i64>,
    pub company_name: Option<String>,
    pub kind_id: Option<i64>,
    pub kind_name: Option<String>,
    pub kind_short_name: Option<String>,
    pub topic: Option<String>,
    pub date_of_practice: Option<NaiveDate>,
    pub date_str: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct PracticeShort {
    pub id: i64,
    pub company_id: Option<i64>,
    pub company_name: Option<String>,
    pub kind_id: Option<i64>,
    pub kind_short_name: Option<String>,
    pub date_of_practice: Option<NaiveDate>,
}

impl Practice {
    // pub fn new() -> Self {
    //     Default::default()
    // }

    pub async fn get(client: &Client, id: i64) -> Result<Practice, ServiceError> {
        let stmt = client
            .prepare(
                "
                    SELECT
                        company_id,
                        kind_id,
                        topic,
                        date_of_practice,
                        note,
                        created_at,
                        updated_at
                    FROM
                        practices
                    WHERE
                        id = $1
                ",
            )
            .await?;
        let row = client.query_one(&stmt, &[&id]).await?;
        let practice = Practice {
            id,
            company_id: row.try_get(0)?,
            kind_id: row.try_get(1)?,
            topic: row.try_get(2)?,
            date_of_practice: row.try_get(3)?,
            note: row.try_get(4)?,
            created_at: row.try_get(5)?,
            updated_at: row.try_get(6)?,
        };
        Ok(practice)
    }

    pub async fn insert(client: &Client, practice: Practice) -> Result<Practice, ServiceError> {
        let mut practice = practice;
        let stmt = client
            .prepare(
                "
                    INSERT INTO practices
                    (
                        company_id,
                        kind_id,
                        topic,
                        date_of_practice,
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
                        $5,
                        $6,
                        $7
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
                    &practice.company_id,
                    &practice.kind_id,
                    &practice.topic,
                    &practice.date_of_practice,
                    &practice.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .await?;
        practice.id = row.get(0);
        Ok(practice)
    }

    pub async fn update(client: &Client, practice: Practice) -> Result<u64, ServiceError> {
        let stmt = client
            .prepare(
                "
                    UPDATE practices SET
                        company_id = $2,
                        kind_id = $3,
                        topic = $4,
                        date_of_practice = $5,
                        note = $6,
                        updated_at = $7
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client
            .execute(
                &stmt,
                &[
                    &practice.id,
                    &practice.company_id,
                    &practice.kind_id,
                    &practice.topic,
                    &practice.date_of_practice,
                    &practice.note,
                    &Local::now().naive_local(),
                ],
            )
            .await?)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<u64, ServiceError> {
        let stmt = client
            .prepare(
                "
                    DELETE FROM
                        practices
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client.execute(&stmt, &[&id]).await?)
    }
}

impl PracticeList {
    pub async fn get_all(client: &Client) -> Result<Vec<PracticeList>, ServiceError> {
        let mut practices = Vec::new();
        let stmt = client
            .prepare(
                "
                    SELECT
                        p.id,
                        p.company_id,
                        c.name AS company_name,
                        p.kind_id,
                        k.name AS kind_name,
                        k.short_name AS kind_short_name,
                        p.date_of_practice,
                        p.topic
                    FROM
                        practices AS p
                    LEFT JOIN
                        companies AS c ON c.id = p.company_id
                    LEFT JOIN
                        kinds AS k ON k.id = p.kind_id
                    ORDER BY 
                        p.date_of_practice DESC
                ",
            )
            .await?;
        for row in client.query(&stmt, &[]).await? {
            let date: Option<NaiveDate> = row.get(6);
            practices.push(PracticeList {
                id: row.try_get(0)?,
                company_id: row.try_get(1)?,
                company_name: row.try_get(2)?,
                kind_id: row.try_get(3)?,
                kind_name: row.try_get(4)?,
                kind_short_name: row.try_get(5)?,
                date_of_practice: row.try_get(6)?,
                topic: row.try_get(7)?,
                date_str: if let Some(d) = date {
                    Some(d.format("%d.%m.%y").to_string())
                } else {
                    None
                },
            });
        }
        Ok(practices)
    }

    pub async fn get_by_company(
        client: &Client,
        company_id: i64,
    ) -> Result<Vec<PracticeList>, ServiceError> {
        let mut practices = Vec::new();
        let stmt = client
            .prepare(
                "
                    SELECT
                        p.id,
                        p.company_id,
                        c.name AS company_name,
                        p.kind_id,
                        k.name AS kind_name,
                        k.short_name AS kind_short_name,
                        p.date_of_practice,
                        p.topic
                    FROM
                        practices AS p
                    LEFT JOIN
                        companies AS c ON c.id = p.company_id
                    LEFT JOIN
                        kinds AS k ON k.id = p.kind_id
                    WHERE
                        p.company_id = $1
                    ORDER BY
                        date_of_practice DESC
                ",
            )
            .await?;
        for row in client.query(&stmt, &[&company_id]).await? {
            let date: Option<NaiveDate> = row.get(6);
            practices.push(PracticeList {
                id: row.try_get(0)?,
                company_id: row.try_get(1)?,
                company_name: row.try_get(2)?,
                kind_id: row.try_get(3)?,
                kind_name: row.try_get(4)?,
                kind_short_name: row.try_get(5)?,
                date_of_practice: row.try_get(6)?,
                topic: row.try_get(7)?,
                date_str: if let Some(d) = date {
                    Some(d.format("%d.%m.%y").to_string())
                } else {
                    None
                },
            });
        }
        Ok(practices)
    }
}

impl PracticeShort {
    pub async fn get_near(client: &Client) -> Result<Vec<PracticeShort>, ServiceError> {
        let mut practices = Vec::new();
        let stmt = client
            .prepare(
                "
                    SELECT
                        p.id,
                        p.company_id,
                        c.name AS company_name,
                        p.kind_id,
                        k.short_name AS kind_short_name,
                        p.date_of_practice
                    FROM
                        practices AS p
                    LEFT JOIN
                        companies AS c ON c.id = p.company_id
                    LEFT JOIN
                        kinds AS k ON k.id = p.kind_id
                    WHERE
                        p.date_of_practice > TIMESTAMP 'now'::timestamp - '1 month'::interval
                    ORDER BY
                        date_of_practice ASC
                    LIMIT 10
                ",
            )
            .await?;
        for row in client.query(&stmt, &[]).await? {
            practices.push(PracticeShort {
                id: row.try_get(0)?,
                company_id: row.try_get(1)?,
                company_name: row.try_get(2)?,
                kind_id: row.try_get(3)?,
                kind_short_name: row.try_get(4)?,
                date_of_practice: row.try_get(5)?,
            });
        }
        Ok(practices)
    }
}

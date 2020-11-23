use chrono::{Local, NaiveDateTime};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};

use crate::error::ServiceError;

#[derive(Default, Deserialize, Serialize)]
pub struct Rank {
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
pub struct RankList {
    pub id: i64,
    pub name: Option<String>,
    pub note: Option<String>,
}

impl Rank {
    // pub fn new() -> Self {
    //     Default::default()
    // }

    pub async fn get(client: &Client, id: i64) -> Result<Rank, ServiceError> {
        let stmt = client
            .prepare(
                "
                    SELECT
                        name,
                        note,
                        created_at,
                        updated_at
                    FROM
                        ranks
                    WHERE
                        id = $1
                ",
            )
            .await?;
        let row = client.query_one(&stmt, &[&id]).await?;
        let rank = Rank {
            id,
            name: row.try_get(0)?,
            note: row.try_get(1)?,
            created_at: row.try_get(2)?,
            updated_at: row.try_get(3)?,
        };
        Ok(rank)
    }

    pub async fn insert(client: &Client, rank: Rank) -> Result<Rank, ServiceError> {
        let mut rank = rank;
        let stmt = client
            .prepare(
                "
                    INSERT INTO ranks
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
                    &rank.name,
                    &rank.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .await?;
        rank.id = row.get(0);
        Ok(rank)
    }

    pub async fn update(client: &Client, rank: Rank) -> Result<u64, ServiceError> {
        let stmt = client
            .prepare(
                "
                    UPDATE ranks SET
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
                    &rank.id,
                    &rank.name,
                    &rank.note,
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
                        ranks
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client.execute(&stmt, &[&id]).await?)
    }
}

impl RankList {
    pub async fn get_all(client: &Client) -> Result<Vec<RankList>, ServiceError> {
        let mut ranks = Vec::new();
        let stmt = client
            .prepare(
                "
                    SELECT
                        id,
                        name,
                        note
                    FROM
                        ranks
                    ORDER BY
                        name ASC
                ",
            )
            .await?;
        for row in client.query(&stmt, &[]).await? {
            ranks.push(RankList {
                id: row.try_get(0)?,
                name: row.try_get(1)?,
                note: row.try_get(2)?,
            });
        }
        Ok(ranks)
    }
}

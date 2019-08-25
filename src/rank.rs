use chrono::{Local, NaiveDateTime};
use postgres::Connection;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize)]
pub struct RankList {
    pub id: i64,
    pub name: Option<String>,
    pub note: Option<String>,
}

impl Rank {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(conn: &Connection, id: i64) -> Result<Rank, String> {
        let mut rank = Rank::new();
        if id == 0 {
            Ok(rank)
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
                            ranks
                        WHERE
                            id = $1
                    ",
                    &[&id],
                )
                .map_err(|e| format!("rank id {} {}", id, e.to_string()))?
            {
                rank = Rank {
                    id,
                    name: row.get(0),
                    note: row.get(1),
                    created_at: row.get(2),
                    updated_at: row.get(3),
                };
            }
            Ok(rank)
        }
    }

    pub fn post(conn: &Connection, id: i64, rank: Rank) -> Result<Rank, String> {
        if id == 0 {
            Rank::insert(conn, rank)
        } else {
            Rank::update(conn, id, rank)
        }
    }

    pub fn insert(conn: &Connection, rank: Rank) -> Result<Rank, String> {
        let mut rank = rank;
        for row in &conn
            .query(
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
                &[
                    &rank.name,
                    &rank.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .map_err(|e| format!("create rank {} ", e.to_string()))?
        {
            rank.id = row.get(0)
        }
        Ok(rank)
    }

    pub fn update(conn: &Connection, id: i64, rank: Rank) -> Result<Rank, String> {
        let mut rank = rank;
        rank.id = id;
        match &conn.execute(
            "
                UPDATE ranks SET
                    name = $2,
                    note = $3,
                    updated_at = $4
                WHERE
                    id = $1
            ",
            &[
                &rank.id,
                &rank.name,
                &rank.note,
                &Local::now().naive_local(),
            ],
        ) {
            Ok(0) => Err(format!("update rank id {}", id)),
            _ => Ok(rank),
        }
    }

    pub fn delete(conn: &Connection, id: i64) -> bool {
        conn.execute(
            "
                DELETE FROM
                    ranks
                WHERE
                    id = $1
            ",
            &[&id],
        )
        .is_ok()
    }
}

impl RankList {
    pub fn get_all(conn: &Connection) -> Result<Vec<RankList>, String> {
        let mut ranks = Vec::new();
        for row in &conn
            .query(
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
                &[],
            )
            .map_err(|e| format!("rankList all {}", e.to_string()))?
        {
            ranks.push(RankList {
                id: row.get(0),
                name: row.get(1),
                note: row.get(2),
            });
        }
        Ok(ranks)
    }
}

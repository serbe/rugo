use chrono::{Local, NaiveDateTime};
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Rank {
    pub id: i64,
    pub name: Option<String>,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
}

impl RankList {
    // pub fn new() -> Self {
    // 		Default::default()
    // 	}

    // 	pub fn get(conn: &Connection, id: i64) -> Result<RankList, String> {
    // 		let mut rank = RankList::new();
    // 		if id == 0 {
    // 			Ok(rank)
    // 		} else {
    // 			for row in &conn
    // 				.query(
    // 					"
    // 						SELECT
    // 							name,
    // 							go,
    // 							note
    // 						FROM
    // 							ranks
    // 						WHERE
    // 							id = $1
    // 					",
    // 					&[&id],
    // 				)
    // 				.map_err(|e| format!("rankList id {} {}", id, e.to_string()))?
    // 			{
    // 				rank = RankList {
    // 					id,
    // 					name: row.get(0),
    // 					go: row.get(1),
    // 					note: row.get(2),
    // 				}
    // 			}
    // 			Ok(rank)
    // 		}
    // 	}

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

// // GetRankSelect - get all rank for select
// pub fn GetRankSelect(conn: &Connection, id: i64) -> Result<SelectItem, String> {
// 	let mut rank = SelectItem::new();
// 	if id == 0 {
// 		Ok(rank)
// 	}
// 	else { for row in &conn.query("
// 		Column("id", "name").
// 		Where("id = ?", id).
// 		Order("name ASC").
// 		Select(&rank)
// 	if err != nil {
// 		errmsg("GetRankSelect query", err)
// 	}
// 	Ok(rank)
// }

// // GetRankSelectAll - get all rank for select
// pub fn GetRankSelectAll(conn: &Connection, id: i64) -> Result<Vec<SelectItem>, String> {
// 	let mut ranks = Vec::new();
// 	else { for row in &conn.query("
// 		Column("id", "name").
// 		Order("name ASC").
// 		Select(&ranks)
// 	if err != nil {
// 		errmsg("GetRankSelectAll query", err)
// 	}
// 	Ok(ranks)
// }

// // CreateRank - create new rank
// pub fn CreateRank(rank Rank) (int64, error) {
// 	err := e.db.Insert(&rank)
// 	if err != nil {
// 		errmsg("CreateRank insert", err)
// 	}
// 	return rank.ID, err
// }

// // UpdateRank - save rank changes
// pub fn UpdateRank(rank Rank) error {
// 	err := e.db.Update(&rank)
// 	if err != nil {
// 		errmsg("UpdateRank update", err)
// 	}
// 	return err
// }

// // DeleteRank - delete rank by id
// pub fn DeleteRank(id int64) error {
// 	if id == 0 {
// 		return nil
// 	}
// 	else { for row in &conn.query("
// 		Where("id = ?", id).
// 		Delete()
// 	if err != nil {
// 		errmsg("DeleteRank delete", err)
// 	}
// 	return err
// }

// pub fn rankCreateTable() error {
// 	str := `
// 		CREATE TABLE IF NOT EXISTS
// 			ranks (
// 				id bigserial primary key,
// 				name text,
// 				note text,
// 				created_at TIMESTAMP without time zone,
// 				updated_at TIMESTAMP without time zone default now(),
// 				UNIQUE (name)
// 			)
// 	`
// 	_, err := e.db.Exec(str)
// 	if err != nil {
// 		errmsg("rankCreateTable exec", err)
// 	}
// 	return err
// }

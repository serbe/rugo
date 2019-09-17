use chrono::{Local, NaiveDate, NaiveDateTime};
use postgres::Connection;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize)]
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

#[derive(Serialize)]
pub struct PracticeShort {
    pub id: i64,
    pub company_id: Option<i64>,
    pub company_name: Option<String>,
    pub kind_id: Option<i64>,
    pub kind_short_name: Option<String>,
    pub date_of_practice: Option<NaiveDate>,
}

impl Practice {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(conn: &Connection, id: i64) -> Result<Practice, String> {
        let mut practice = Practice::new();
        if id == 0 {
            Ok(practice)
        } else {
            for row in &conn
                .query(
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
                    &[&id],
                )
                .map_err(|e| format!("practice id {} {}", id, e.to_string()))?
            {
                practice = Practice {
                    id,
                    company_id: row.get(0),
                    kind_id: row.get(1),
                    topic: row.get(2),
                    date_of_practice: row.get(3),
                    note: row.get(4),
                    created_at: row.get(5),
                    updated_at: row.get(6),
                };
            }
            Ok(practice)
        }
    }

    pub fn post(conn: &Connection, id: i64, practice: Practice) -> Result<Practice, String> {
        if id == 0 {
            Practice::insert(conn, practice)
        } else {
            Practice::update(conn, id, practice)
        }
    }

    pub fn insert(conn: &Connection, practice: Practice) -> Result<Practice, String> {
        let mut practice = practice;
        for row in &conn
            .query(
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
            .map_err(|e| format!("create practice {} ", e.to_string()))?
        {
            practice.id = row.get(0)
        }
        Ok(practice)
    }

    pub fn update(conn: &Connection, id: i64, practice: Practice) -> Result<Practice, String> {
        let mut practice = practice;
        practice.id = id;
        match &conn.execute(
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
            &[
                &practice.id,
                &practice.company_id,
                &practice.kind_id,
                &practice.topic,
                &practice.date_of_practice,
                &practice.note,
                &Local::now().naive_local(),
            ],
        ) {
            Ok(0) => Err(format!("update practice id {}", id)),
            _ => Ok(practice),
        }
    }

    pub fn delete(conn: &Connection, id: i64) -> bool {
        conn.execute(
            "
                DELETE FROM
                    practices
                WHERE
                    id = $1
            ",
            &[&id],
        )
        .is_ok()
    }
}

impl PracticeList {
    pub fn get_all(conn: &Connection) -> Result<Vec<PracticeList>, String> {
        let mut practices = Vec::new();
        for row in &conn
            .query(
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
                &[],
            )
            .map_err(|e| format!("practice list all {}", e.to_string()))?
        {
            let date: Option<NaiveDate> = row.get(6);
            practices.push(PracticeList {
                id: row.get(0),
                company_id: row.get(1),
                company_name: row.get(2),
                kind_id: row.get(3),
                kind_name: row.get(4),
                kind_short_name: row.get(5),
                date_of_practice: row.get(6),
                topic: row.get(7),
                date_str: if let Some(d) = date {
                    Some(d.format("%d.%m.%y").to_string())
                } else {
                    None
                },
            });
        }
        Ok(practices)
    }

    pub fn get_by_company(conn: &Connection, company_id: i64) -> Result<Vec<PracticeList>, String> {
        let mut practices = Vec::new();
        for row in &conn
            .query(
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
                &[&company_id],
            )
            .map_err(|e| format!("practice list by company {}", e.to_string()))?
        {
            let date: Option<NaiveDate> = row.get(6);
            practices.push(PracticeList {
                id: row.get(0),
                company_id: row.get(1),
                company_name: row.get(2),
                kind_id: row.get(3),
                kind_name: row.get(4),
                kind_short_name: row.get(5),
                date_of_practice: row.get(6),
                topic: row.get(7),
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
    pub fn get_near(conn: &Connection) -> Result<Vec<PracticeShort>, String> {
        let mut practices = Vec::new();
        for row in &conn
            .query(
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
                        date_of_practice DESC
                    LIMIT 10
                ",
                &[],
            )
            .map_err(|e| format!("practece list near {}", e.to_string()))?
        {
            practices.push(PracticeShort {
                id: row.get(0),
                company_id: row.get(1),
                company_name: row.get(2),
                kind_id: row.get(3),
                kind_short_name: row.get(4),
                date_of_practice: row.get(5),
            });
        }
        Ok(practices)
    }
}

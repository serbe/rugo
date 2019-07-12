use chrono::{NaiveDate, NaiveDateTime};
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Practice {
    pub id: i64,
    pub company_id: Option<i64>,
    pub company_name: Option<String>,
    pub kind_id: Option<i64>,
    pub kind_name: Option<String>,
    pub topic: Option<String>,
    pub date_of_practice: Option<NaiveDate>,
    pub note: Option<String>,
    pub created_at: Option<NaiveDateTime>,
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
                            p.id,
                            p.company_id,
                            c.name AS company_name,
                            p.kind_id,
                            k.name AS kind_name,
                            p.topic
                            p.date_of_practice,
                            p.note,
                            created_at,
                            updated_at
                        FROM
                            practices AS p
                        LEFT JOIN
                            companies AS c ON c.id = p.company_id
                        LEFT JOIN
                            kinds AS k ON k.id = p.kind_id
                        WHERE
                            id = $1
                    ",
                    &[&id],
                )
                .map_err(|e| format!("practice {}", e.to_string()))?
            {
                practice = Practice {
                    id: row.get(0),
                    company_id: row.get(1),
                    company_name: row.get(2),
                    kind_id: row.get(3),
                    kind_name: row.get(4),
                    topic: row.get(5),
                    date_of_practice: row.get(6),
                    note: row.get(7),
                    created_at: row.get(8),
                    updated_at: row.get(9),
                };
            }
            Ok(practice)
        }
    }
}

impl PracticeList {
    // pub fn new() -> Self {
    //     Default::default()
    // }

    // pub fn get(conn: &Connection, id: i64) -> Result<PracticeList, String> {
    //     let mut practice = PracticeList::new();
    //     if id == 0 {
    //         Ok(practice)
    //     } else {
    //         for row in &conn
    //             .query(
    //                 "
    //                     SELECT
    //                         p.id,
    //                         p.company_id,
    //                         c.name AS company_name,
    //                         p.kind_id,
    //                         k.name AS kind_name,
    //                         k.short_name AS kind_short_name,
    //                         p.date_of_practice,
    //                         p.topic
    //                     FROM
    //                         practices AS p
    //                     LEFT JOIN
    //                         companies AS c ON c.id = p.company_id
    //                     LEFT JOIN
    //                         kinds AS k ON k.id = p.kind_id
    //                     WHERE
    //                         id = $1
    //                 ",
    //                 &[&id],
    //             )
    //             .map_err(|e| format!("practice list by id {}", e.to_string()))?
    //         {
    //             let date: Option<NaiveDate> = row.get(6);
    //             practice = PracticeList {
    //                 id: row.get(0),
    //                 company_id: row.get(1),
    //                 company_name: row.get(2),
    //                 kind_id: row.get(3),
    //                 kind_name: row.get(4),
    //                 kind_short_name: row.get(5),
    //                 date_of_practice: row.get(6),
    //                 topic: row.get(7),
    //                 date_str: if let Some(d) = date {
    //                     Some(d.format("%d.%m.%y").to_string())
    //                 } else {
    //                     None
    //                 },
    //             };
    //         }
    //         Ok(practice)
    //     }
    // }

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

// // CreatePractice - create new practice
// pub fn CreatePractice(practice Practice) (int64, error) {
// 	err := e.db.Insert(&practice)
// 	if err != nil {
// 		errmsg("CreatePractice insert", err)
// 	}
// 	return practice.ID, err
// }

// // UpdatePractice - save practice changes
// pub fn UpdatePractice(practice Practice) error {
// 	err := e.db.Update(&practice)
// 	if err != nil {
// 		errmsg("UpdatePractice update", err)
// 	}
// 	return err
// }

// // DeletePractice - delete practice by id
// pub fn DeletePractice(id int64) error {
// 	if id == 0 {
// 		return nil
// 	}
// 	else { for row in &conn.query("
// 		Where("id = ?", id).
// 		Delete()
// 	if err != nil {
// 		errmsg("DeletePractice delete", err)
// 	}
// 	return err
// }

// pub fn practiceCreateTable() error {
// 	str := `
// 		CREATE TABLE IF NOT EXISTS
// 			practices (
// 				id bigserial primary key,
// 				company_id bigint,
// 				kind_id bigint,
// 				topic text,
// 				date_of_practice date,
// 				note text,
// 				created_at TIMESTAMP without time zone,
// 				updated_at TIMESTAMP without time zone default now()
// 			)
// 	`
// 	_, err := e.db.Exec(str)
// 	if err != nil {
// 		errmsg("practiceCreateTable exec", err)
// 	}
// 	return err
// }

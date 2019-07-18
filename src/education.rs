use chrono::{NaiveDate, NaiveDateTime};
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Education {
    pub id: i64,
    pub contact_id: Option<i64>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub start_str: Option<String>,
    pub end_str: Option<String>,
    pub post_id: Option<i64>,
    pub note: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize)]
pub struct EducationList {
    pub id: i64,
    pub contact_id: Option<i64>,
    pub contact_name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub start_str: Option<String>,
    pub end_str: Option<String>,
    pub post_id: Option<i64>,
    pub post_name: Option<String>,
    pub note: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct EducationShort {
    pub id: i64,
    pub contact_id: Option<i64>,
    pub contact_name: Option<String>,
    pub start_date: Option<NaiveDate>,
}

impl Education {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(conn: &Connection, id: i64) -> Result<Education, String> {
        let mut education = Education::new();
        if id == 0 {
            Ok(education)
        } else {
            for row in &conn
                .query(
                    "
						SELECT
							contact_id,
							start_date,
							end_date,
							post_id,
							note,
							created_at,
							updated_at,
						FROM
							educations
						WHERE
							id = $1
					",
                    &[&id],
                )
                .map_err(|e| format!("education id {} {}", id, e.to_string()))?
            {
                let start_date: Option<NaiveDate> = row.get(1);
                let end_date: Option<NaiveDate> = row.get(2);
                education = Education {
                    id,
                    contact_id: row.get(0),
                    start_date: row.get(1),
                    end_date: row.get(2),
                    start_str: if let Some(d) = start_date {
                        Some(d.format("%Y-%m-%d").to_string())
                    } else {
                        None
                    },
                    end_str: if let Some(d) = end_date {
                        Some(d.format("%Y-%m-%d").to_string())
                    } else {
                        None
                    },
                    post_id: row.get(3),
                    note: row.get(4),
                    created_at: row.get(5),
                    updated_at: row.get(6),
                };
            }
            Ok(education)
        }
    }

    //     pub fn get_all(conn: &Connection) -> Result<Vec<Education>, String> {
    //         let mut educations = Vec::new();
    //         for row in &conn
    //             .query(
    //                 "
    // 					SELECT
    // 						id,
    // 						contact_id,
    // 						start_date,
    // 						end_date,
    // 						post_id,
    // 						note,
    // 						created_at,
    // 						updated_at,
    // 					FROM
    // 						educations
    // 					ORDER BY
    // 						start_date
    // 				",
    //                 &[],
    //             )
    //             .map_err(|e| format!("education all {}", e.to_string()))?
    //         {
    //             let start_date: Option<NaiveDate> = row.get(2);
    //             let end_date: Option<NaiveDate> = row.get(3);
    //             educations.push(Education {
    //                 id: row.get(0),
    //                 contact_id: row.get(1),
    //                 start_date: row.get(2),
    //                 end_date: row.get(3),
    //                 start_str: if let Some(d) = start_date {
    //                     Some(d.format("%Y-%m-%d").to_string())
    //                 } else {
    //                     None
    //                 },
    //                 end_str: if let Some(d) = end_date {
    //                     Some(d.format("%Y-%m-%d").to_string())
    //                 } else {
    //                     None
    //                 },
    //                 post_id: row.get(4),
    //                 note: row.get(5),
    //                 created_at: row.get(6),
    //                 updated_at: row.get(7),
    //             });
    //         }
    //         Ok(educations)
    //     }
}

impl EducationList {
    pub fn get_all(conn: &Connection) -> Result<Vec<EducationList>, String> {
        let mut educations = Vec::new();
        for row in &conn
            .query(
                "
					SELECT
						e.id,
						e.contact_id,
						c.name AS contact_name,
						e.start_date,
						e.end_date,
						e.post_id,
						p.name AS post_name,
						e.note
					FROM
						educations AS e
					LEFT JOIN
						contacts AS c ON c.id = e.contact_id
					LEFT JOIN
						posts AS p ON p.id = e.post_id
					ORDER BY
						start_date DESC
				",
                &[],
            )
            .map_err(|e| format!("educations list all {}", e.to_string()))?
        {
            let start_str: Option<NaiveDate> = row.get(3);
            let end_str: Option<NaiveDate> = row.get(4);
            educations.push(EducationList {
                id: row.get(0),
                contact_id: row.get(1),
                contact_name: row.get(2),
                start_date: row.get(3),
                end_date: row.get(4),
                start_str: if let Some(d) = start_str {
                    Some(d.format("%Y-%m-%d").to_string())
                } else {
                    None
                },
                end_str: if let Some(d) = end_str {
                    Some(d.format("%Y-%m-%d").to_string())
                } else {
                    None
                },
                post_id: row.get(5),
                post_name: row.get(6),
                note: row.get(7),
            });
        }
        Ok(educations)
    }
}

impl EducationShort {
    pub fn get_near(conn: &Connection) -> Result<Vec<EducationShort>, String> {
        let mut educations = Vec::new();
        for row in &conn
            .query(
                "
					SELECT
						e.id,
						e.contact_id,
						c.name AS contact_name,
						e.start_date
					FROM
						educations AS e
					LEFT JOIN
						contacts AS c ON c.id = e.contact_id
					WHERE
						e.start_date > TIMESTAMP 'now'::timestamp - '1 month'::interval
					ORDER BY
						start_date ASC
					LIMIT 10
				",
                &[],
            )
            .map_err(|e| format!("educations list near {}", e.to_string()))?
        {
            educations.push(EducationShort {
                id: row.get(0),
                contact_id: row.get(1),
                contact_name: row.get(2),
                start_date: row.get(3),
            });
        }
        Ok(educations)
    }
}

// // CreateEducation - create new education
// pub fn CreateEducation(education Education) (int64, error) {
// 	err := e.db.Insert(&education)
// 	if err != nil {
// 		errmsg("CreateEducation insert", err)
// 	}
// 	return education.ID, err
// }

// // UpdateEducation - save changes to education
// pub fn UpdateEducation(education Education) error {
// 	err := e.db.Update(&education)
// 	if err != nil {
// 		errmsg("UpdateEducation update", err)
// 	}
// 	return err
// }

// // DeleteEducation - delete education by id
// pub fn DeleteEducation(id int64) error {
// 	if id == 0 {
// 		return nil
// 	}
// 	else { for row in &conn.query("
// 		Where("id = ?", id).
// 		Delete()
// 	if err != nil {
// 		errmsg("DeleteEducation delete", err)
// 	}
// 	return err
// }

// pub fn educationCreateTable() error {
// 	str := `
// 		CREATE TABLE IF NOT EXISTS
// 			educations (
// 				id bigserial primary key,
// 				start_date date,
// 				end_date date,
// 				note text,
// 				post_id bigint,
// 				created_at TIMESTAMP without time zone,
// 				updated_at TIMESTAMP without time zone default now()
// 			)
// 	`
// 	_, err := e.db.Exec(str)
// 	if err != nil {
// 		errmsg("educationCreateTable exec", err)
// 	}
// 	return err
// }

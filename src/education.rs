use chrono::{Local, NaiveDate, NaiveDateTime};
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Education {
    #[serde(default)]
    pub id: i64,
    pub contact_id: Option<i64>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub post_id: Option<i64>,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize)]
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

#[derive(Serialize)]
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
							updated_at
						FROM
							educations
						WHERE
							id = $1
					",
                    &[&id],
                )
                .map_err(|e| format!("education id {} {}", id, e.to_string()))?
            {
                education = Education {
                    id,
                    contact_id: row.get(0),
                    start_date: row.get(1),
                    end_date: row.get(2),
                    post_id: row.get(3),
                    note: row.get(4),
                    created_at: row.get(5),
                    updated_at: row.get(6),
                };
            }
            Ok(education)
        }
    }

    pub fn post(conn: &Connection, id: i64, education: Education) -> Result<Education, String> {
        if id == 0 {
            Education::insert(conn, education)
        } else {
            Education::update(conn, id, education)
        }
    }

    pub fn insert(conn: &Connection, education: Education) -> Result<Education, String> {
        let mut education = education;
        for row in &conn
            .query(
                "
                    INSERT INTO educations
                    (
                        contact_id,
                        start_date,
                        end_date,
                        post_id,
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
                    &education.contact_id,
                    &education.start_date,
                    &education.end_date,
                    &education.post_id,
                    &education.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .map_err(|e| format!("create education {} ", e.to_string()))?
        {
            education.id = row.get(0)
        }
        Ok(education)
    }

    pub fn update(conn: &Connection, id: i64, education: Education) -> Result<Education, String> {
        let mut education = education;
        education.id = id;
        match &conn.execute(
            "
                UPDATE educations SET
                    contact_id = $2,
                    start_date = $3,
                    end_date = $4,
                    post_id = $5,
                    note = $6,
                    updated_at = $7
                WHERE
                    id = $1
            ",
            &[
                &education.id,
                &education.contact_id,
                &education.start_date,
                &education.end_date,
                &education.post_id,
                &education.note,
                &Local::now().naive_local(),
            ],
        ) {
            Ok(0) => Err(format!("update education id {}", id)),
            _ => Ok(education),
        }
    }

    pub fn delete(conn: &Connection, id: i64) -> bool {
        conn.execute(
            "
                DELETE FROM
                    educations
                WHERE
                    id = $1
            ",
            &[&id],
        )
        .is_ok()
    }
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

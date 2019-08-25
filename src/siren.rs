use chrono::{Local, NaiveDateTime};
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Siren {
    #[serde(default)]
    pub id: i64,
    pub num_id: Option<i64>,
    pub num_pass: Option<String>,
    pub siren_type_id: Option<i64>,
    pub address: Option<String>,
    pub radio: Option<String>,
    pub desk: Option<String>,
    pub contact_id: Option<i64>,
    pub company_id: Option<i64>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub stage: Option<i64>,
    pub own: Option<String>,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SirenList {
    pub id: i64,
    pub siren_type_name: Option<String>,
    pub address: Option<String>,
    pub contact_name: Option<String>,
    pub phones: Option<Vec<i64>>,
}

impl Siren {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(conn: &Connection, id: i64) -> Result<Siren, String> {
        let mut siren = Siren::new();
        if id == 0 {
            Ok(siren)
        } else {
            for row in &conn
                .query(
                    "
                        SELECT
                            num_id,
                            num_pass,
                            siren_type_id,
                            address,
                            radio,
                            desk,
                            contact_id,
                            company_id,
                            latitude,
                            longitude,
                            stage,
                            own,
                            note,
                            created_at,
                            updated_at
                        FROM
                            sirens
                        WHERE
                            id = $1
                    ",
                    &[&id],
                )
                .map_err(|e| format!("siren id {} {}", id, e.to_string()))?
            {
                siren = Siren {
                    id,
                    num_id: row.get(0),
                    num_pass: row.get(1),
                    siren_type_id: row.get(2),
                    address: row.get(3),
                    radio: row.get(4),
                    desk: row.get(5),
                    contact_id: row.get(6),
                    company_id: row.get(7),
                    latitude: row.get(8),
                    longitude: row.get(9),
                    stage: row.get(10),
                    own: row.get(11),
                    note: row.get(12),
                    created_at: row.get(13),
                    updated_at: row.get(14),
                };
            }
            Ok(siren)
        }
    }

    pub fn post(conn: &Connection, id: i64, siren: Siren) -> Result<Siren, String> {
        if id == 0 {
            Siren::insert(conn, siren)
        } else {
            Siren::update(conn, id, siren)
        }
    }

    pub fn insert(conn: &Connection, siren: Siren) -> Result<Siren, String> {
        let mut siren = siren;
        for row in &conn
            .query(
                "
                    INSERT INTO sirens
                    (
                        num_id,
                        num_pass,
                        siren_type_id,
                        address,
                        radio,
                        desk,
                        contact_id,
                        company_id,
                        latitude,
                        longitude,
                        stage,
                        own,
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
                        $7,
                        $8,
                        $9,
                        $10,
                        $11,
                        $12,
                        $13,
                        $14,
                        $15
                    )
                    RETURNING
                        id
                ",
                &[
                    &siren.num_id,
                    &siren.num_pass,
                    &siren.siren_type_id,
                    &siren.address,
                    &siren.radio,
                    &siren.desk,
                    &siren.contact_id,
                    &siren.company_id,
                    &siren.latitude,
                    &siren.longitude,
                    &siren.stage,
                    &siren.own,
                    &siren.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .map_err(|e| format!("create siren {} ", e.to_string()))?
        {
            siren.id = row.get(0)
        }
        Ok(siren)
    }

    pub fn update(conn: &Connection, id: i64, siren: Siren) -> Result<Siren, String> {
        let mut siren = siren;
        siren.id = id;
        match &conn.execute(
            "
                UPDATE sirens SET
                    num_id = $2,
                    num_pass = $3,
                    siren_type_id = $4,
                    address = $5,
                    radio = $6,
                    desk = $7,
                    contact_id = $8,
                    company_id = $9,
                    latitude = $10,
                    longitude = $11,
                    stage = $12,
                    own = $13,
                    note = $14,
                    updated_at = $15
                WHERE
                    id = $1
            ",
            &[
                &siren.id,
                &siren.num_id,
                &siren.num_pass,
                &siren.siren_type_id,
                &siren.address,
                &siren.radio,
                &siren.desk,
                &siren.contact_id,
                &siren.company_id,
                &siren.latitude,
                &siren.longitude,
                &siren.stage,
                &siren.own,
                &siren.note,
                &Local::now().naive_local(),
            ],
        ) {
            Ok(0) => Err(format!("update siren id {}", id)),
            _ => Ok(siren),
        }
    }

    pub fn delete(conn: &Connection, id: i64) -> bool {
        conn.execute(
            "
                DELETE FROM
                    sirens
                WHERE
                    id = $1
            ",
            &[&id],
        )
        .is_ok()
    }
}

impl SirenList {
    pub fn get_all(conn: &Connection) -> Result<Vec<SirenList>, String> {
        let mut sirens = Vec::new();
        for row in &conn
            .query(
                "
					SELECT
						s.id,
						s.address,
						t.name AS siren_type_name,
						c.name AS contact_name,
						array_agg(DISTINCT ph.phone) AS phones
					FROM
						sirens AS s
					LEFT JOIN
						siren_types AS t ON s.siren_type_id = t.id
					LEFT JOIN
						contacts AS c ON s.contact_id = c.id
					LEFT JOIN
						phones AS ph ON s.contact_id = ph.contact_id AND ph.fax = false
					GROUP BY
						s.id,
						t.id,
						c.id
					ORDER BY
						t.name ASC
				",
                &[],
            )
            .map_err(|e| format!("sirenList all {}", e.to_string()))?
        {
            let phones = match row.get_opt(4) {
                Some(Ok(data)) => Some(data),
                _ => None,
            };
            sirens.push(SirenList {
                id: row.get(0),
                siren_type_name: row.get(1),
                address: row.get(2),
                contact_name: row.get(3),
                phones,
            });
        }
        Ok(sirens)
    }
}

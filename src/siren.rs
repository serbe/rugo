use chrono::NaiveDateTime;
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Siren {
    pub id: i64,
    pub num_id: Option<i64>,
    pub num_pass: Option<String>,
    pub type_id: Option<i64>,
    pub siren_type_id: Option<i64>,
    pub siren_type_name: Option<String>,
    pub address: Option<String>,
    pub radio: Option<String>,
    pub desk: Option<String>,
    pub contact_id: Option<i64>,
    pub contact_name: Option<String>,
    pub company_id: Option<i64>,
    pub company_name: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub stage: Option<i64>,
    pub own: Option<String>,
    pub note: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize)]
pub struct SirenList {
    pub id: i64,
    pub siren_type_name: Option<String>,
    pub address: Option<String>,
    pub contact_name: Option<String>,
    pub phones: Option<Vec<i64>>,
}

// // GetSiren - get one siren by id
// pub fn GetSiren(conn: &Connection, id: i64) -> Result<Siren, String> {
// 	let mut siren = Siren::new();
// 	if id == 0 {
// 		Ok(siren)
// 	}
// 	else { for row in &conn.query("
// 		Where("id = ?", id).
// 		.map_err(|e| format!(" id {} {}", id, e.to_string()))? {
// 	if err != nil {
// 		errmsg("GetSiren select", err)
// 	}
// 	Ok(siren)
// }

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
						siren_types AS t ON s.type_id = t.id
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

// // CreateSiren - create new siren
// pub fn CreateSiren(siren Siren) (int64, error) {
// 	err := e.db.Insert(&siren)
// 	if err != nil {
// 		errmsg("CreateSiren insert", err)
// 	}
// 	return siren.ID, err
// }

// // UpdateSiren - save siren changes
// pub fn UpdateSiren(siren Siren) error {
// 	err := e.db.Update(&siren)
// 	if err != nil {
// 		errmsg("UpdateSiren update", err)
// 	}
// 	return err
// }

// // DeleteSiren - delete siren by id
// pub fn DeleteSiren(id int64) error {
// 	if id == 0 {
// 		return nil
// 	}
// 	else { for row in &conn.query("
// 		Where("id = ?", id).
// 		Delete()
// 	if err != nil {
// 		errmsg("DeleteSiren delete", err)
// 	}
// 	return err
// }

// pub fn sirenCreateTable() error {
// 	str := `
// 		CREATE TABLE IF NOT EXISTS
// 			sirens (
// 				id         bigserial PRIMARY KEY,
// 				num_id     bigint,
// 				num_pass   text,
// 				type_id    bigint,
// 				address    text,
// 				radio      text,
// 				desk       text,
// 				contact_id bigint,
// 				company_id bigint,
// 				latitude   text,
// 				longitude  text,
// 				stage      bigint,
// 				own        text,
// 				note        text,
// 				created_at TIMESTAMP without time zone,
// 				updated_at TIMESTAMP without time zone default now(),
// 				UNIQUE(num_id, num_pass, type_id)
// 			)
// 	`
// 	_, err := e.db.Exec(str)
// 	if err != nil {
// 		errmsg("sirenCreateTable exec", err)
// 	}
// 	return err
// }

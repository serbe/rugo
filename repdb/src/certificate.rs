use chrono::{NaiveDate, NaiveDateTime};
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Certificate {
    pub id: i64,
    pub num: Option<String>,
    pub contact_id: Option<i64>,
    pub contact_name: Option<String>,
    pub company_id: Option<i64>,
    pub company_name: Option<String>,
    pub cert_date: Option<NaiveDate>,
    pub note: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize)]
pub struct CertificateList {
    pub id: i64,
    pub num: Option<String>,
    pub contact_id: Option<i64>,
    pub contact_name: Option<String>,
    pub company_id: Option<i64>,
    pub company_name: Option<String>,
    pub cert_date: Option<String>,
}

impl Certificate {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(conn: &Connection, id: i64) -> Result<Certificate, String> {
        let mut certificate = Certificate::new();
        if id == 0 {
            Ok(certificate)
        } else {
            for row in &conn
                .query(
                    "
                        SELECT
                            c.num,
                            c.contact_id,
                            cn.name AS contact_name,
                            c.company_id,
                            co.name AS company_name,
                            c.cert_date,
                            c.note,
                            c.created_at,
                            c.updated_at
                        FROM
                            certificates AS c
                        LEFT JOIN
                            contacts AS cn ON c.contact_id = cn.id
                        LEFT JOIN
                            companies AS co ON c.company_id = co.id
                        WHERE
                            c.id = $1
                    ",
                    &[&id],
                )
                .map_err(|e| format!("certificate id {} - {}", id, e.to_string()))?
            {
                certificate.id = id;
                certificate.num = row.get(0);
                certificate.contact_id = row.get(1);
                certificate.contact_name = row.get(2);
                certificate.company_id = row.get(3);
                certificate.company_name = row.get(4);
                certificate.cert_date = row.get(5);
                certificate.note = row.get(6);
                certificate.created_at = row.get(7);
                certificate.updated_at = row.get(8);
            }
            Ok(certificate)
        }
    }
}

impl CertificateList {
    pub fn get_all(conn: &Connection) -> Result<Vec<CertificateList>, String> {
        let mut certificates = Vec::new();
        for row in &conn
            .query(
                "
                    SELECT
                        c.id,
                        c.num,
                        c.contact_id,
                        p.name AS contact_name,
                        c.company_id,
                        co.name AS company_name,
                        c.cert_date
                    FROM
                        certificates AS c
                    LEFT JOIN
                        contacts AS p ON c.contact_id = p.id
                    LEFT JOIN
                        companies AS co ON c.company_id = co.id
                    GROUP BY
                        c.id,
                        p.name,
                        co.name
                    ORDER BY
                        num ASC
                ",
                &[],
            )
            .map_err(|e| format!("Certificate list {}", e.to_string()))?
        {
            let date: Option<NaiveDate> = row.get(6);
            certificates.push(CertificateList {
                id: row.get("id"),
                num: row.get("num"),
                contact_id: row.get(2),
                contact_name: row.get(3),
                company_id: row.get(4),
                company_name: row.get(5),
                cert_date: if let Some(d) = date {
                    Some(d.format("%Y-%m-%d").to_string())
                } else {
                    None
                },
            });
        }
        Ok(certificates)
    }
}

// fn create_certificate(conn: Connection, certificate: Certificate) -> Result<u64, String> {
//     let mut id = 0u64;
//     for row in &conn.query("
//         INSERT INTO
//             certificates
//             (
//                 num,
//                 contact_id,
//                 company_id,
//                 cert_date,
//                 note,
//                 created_at,
//                 updated_at
//             )
//         VALUES
//             ($1, $2, $3, $4, $5, $6, $7)
//         RETURNING
//             id
//     ", &[&certificate.num,
//         &certificate.contact_id,
//         &certificate.company_id,
//         &certificate.cert_date,
//         &certificate.note,
//         &certificate.created_at,
//         &certificate.updated_at]).map_err(|e| format!("insert Certificate {}", e.to_string()))? {
//         id = row.get("id");
//     }
//     Ok(id)
// }

// // // UpdateCertificate - save certificate changes
// // pub fn UpdateCertificate(certificate Certificate) error {
// // 	err := e.db.Update(&certificate)
// // 	if err != nil {
// // 		errmsg("UpdateCertificate update", err)
// // 	}
// // 	return err
// // }
// fn update_certificate(conn: Connection, certificate: Certificate) -> Result<i64, String> {
//     &conn.execute("
//         UPDATE
//             certificates
//         SET
//             (
//                 num,
//                 contact_id,
//                 company_id,
//                 cert_date,
//                 note,
//                 created_at,
//                 updated_at
//             )
//         VALUES
//             ($2, $3, $4, $5, $6, $7, $8)
//         WHERE
//             id = $1
//     ", &[&certificate.id,
//         &certificate.num,
//         &certificate.contact_id,
//         &certificate.company_id,
//         &certificate.cert_date,
//         &certificate.note,
//         &certificate.created_at,
//         &certificate.updated_at]).map_err("update Certificate")
// }

// // // DeleteCertificate - delete certificate by id
// // pub fn DeleteCertificate(id int64) error {
// // 	if id == 0 {
// // 		return nil
// // 	}
// // 	else { for row in &conn.query("
// // 		Where("id = ?", id).
// // 		Delete()
// // 	if err != nil {
// // 		errmsg("DeleteCertificate delete", err)
// // 	}
// // 	return err
// // }

// // pub fn certificateCreateTable() error {
// // 	str := `
// // 		CREATE TABLE IF NOT EXISTS
// // 			certificates (
// // 				id BIGSERIAL PRIMARY KEY,
// // 				num TEXT,
// // 				contact_id BIGINT,
// // 				company_id BIGINT,
// // 				cert_date DATE,
// // 				note TEXT,
// // 				created_at TIMESTAMP without time zone,
// // 				updated_at TIMESTAMP without time zone default now(),
// // 				UNIQUE(num)
// // 			)
// // 	`
// // 	_, err := e.db.Exec(str)
// // 	if err != nil {
// // 		errmsg("certificateCreateTable exec", err)
// // 	}
// // 	return err
// // }

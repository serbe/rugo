use chrono::{Local, NaiveDate, NaiveDateTime};
use postgres::Client;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Certificate {
    #[serde(default)]
    pub id: i64,
    pub num: Option<String>,
    pub contact_id: Option<i64>,
    pub company_id: Option<i64>,
    pub cert_date: Option<NaiveDate>,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct CertificateList {
    pub id: i64,
    pub num: Option<String>,
    pub contact_id: Option<i64>,
    pub contact_name: Option<String>,
    pub company_id: Option<i64>,
    pub company_name: Option<String>,
    pub cert_date: Option<String>,
    pub note: Option<String>,
}

impl Certificate {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(conn: &mut Client, id: i64) -> Result<Certificate, String> {
        let mut certificate = Certificate::new();
        if id == 0 {
            Ok(certificate)
        } else {
            for row in &conn
                .query(
                    "
                        SELECT
                            num,
                            contact_id,
                            company_id,
                            cert_date,
                            note,
                            created_at,
                            updated_at
                        FROM
                            certificates
                        WHERE
                            id = $1
                    ",
                    &[&id],
                )
                .map_err(|e| format!("certificate id {} - {}", id, e.to_string()))?
            {
                certificate.id = id;
                certificate.num = row.get(0);
                certificate.contact_id = row.get(1);
                certificate.company_id = row.get(2);
                certificate.cert_date = row.get(3);
                certificate.note = row.get(4);
                certificate.created_at = row.get(5);
                certificate.updated_at = row.get(6);
            }
            Ok(certificate)
        }
    }

    pub fn insert(conn: &mut Client, certificate: Certificate) -> Result<Certificate, String> {
        let mut certificate = certificate;
        for row in &conn
            .query(
                "
                    INSERT INTO certificates
                    (
                        num,
                        contact_id,
                        company_id,
                        cert_date,
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
                    &certificate.num,
                    &certificate.contact_id,
                    &certificate.company_id,
                    &certificate.cert_date,
                    &certificate.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .map_err(|e| format!("create certificate {} ", e.to_string()))?
        {
            certificate.id = row.get(0)
        }
        Ok(certificate)
    }

    pub fn update(conn: &mut Client, certificate: Certificate) -> Result<Certificate, String> {
        match &conn.execute(
            "
                UPDATE certificates SET
                    num = $2,
                    contact_id = $3,
                    company_id = $4,
                    cert_date = $5,
                    note = $6,
                    updated_at = $7
                WHERE
                    id = $1
            ",
            &[
                &certificate.id,
                &certificate.num,
                &certificate.contact_id,
                &certificate.company_id,
                &certificate.cert_date,
                &certificate.note,
                &Local::now().naive_local(),
            ],
        ) {
            Ok(0) => Err(format!("update certificate id {}", certificate.id)),
            _ => Ok(certificate),
        }
    }

    pub fn delete(conn: &mut Client, id: i64) -> bool {
        conn.execute(
            "
                DELETE FROM
                    certificates
                WHERE
                    id = $1
            ",
            &[&id],
        )
        .is_ok()
    }
}

impl CertificateList {
    pub fn get_all(conn: &mut Client) -> Result<Vec<CertificateList>, String> {
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
                        c.cert_date,
                        c.note
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
                note: row.get(7),
            });
        }
        Ok(certificates)
    }
}

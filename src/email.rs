use chrono::{Local, NaiveDateTime};
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Email {
    pub id: i64,
    pub company_id: Option<i64>,
    pub contact_id: Option<i64>,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

impl Email {
    pub fn new() -> Self {
        Default::default()
    }

    fn insert(conn: &Connection, email: Email) -> Result<u64, String> {
        match &conn.execute(
            "
            INSERT INTO emails
            (
                company_id,
                contact_id,
                email,
                created_at,
                updated_at
            )
            VALUES
            (
                $1,
                $2,
                $3,
                $4,
                $5
            )
        ",
            &[
                &email.company_id,
                &email.contact_id,
                &email.email,
                &Local::now().naive_local(),
                &Local::now().naive_local(),
            ],
        ) {
            Ok(1) => Ok(1),
            _ => Err("failed insert email".to_string()),
        }
    }

    pub fn update_contacts(conn: &Connection, id: i64, emails: Vec<String>) -> Result<u64, String> {
        Email::delete_contacts(conn, id);
        for value in emails {
            let mut email = Email::new();
            email.contact_id = Some(id);
            email.email = Some(value);
            Email::insert(conn, email)?;
        }
        Ok(0)
    }

    pub fn update_companies(
        conn: &Connection,
        id: i64,
        emails: Vec<String>,
    ) -> Result<u64, String> {
        Email::delete_companies(conn, id);
        for value in emails {
            let mut email = Email::new();
            email.company_id = Some(id);
            email.email = Some(value);
            Email::insert(conn, email)?;
        }
        Ok(0)
    }

    pub fn delete_contacts(conn: &Connection, id: i64) {
        let _ = &conn.execute(
            "
            DELETE FROM
                emails
            WHERE
                contact_id = $1
        ",
            &[&id],
        );
    }

    pub fn delete_companies(conn: &Connection, id: i64) {
        let _ = &conn.execute(
            "
            DELETE FROM
                emails
            WHERE
                company_id = $1
        ",
            &[&id],
        );
    }
}

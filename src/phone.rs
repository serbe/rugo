use chrono::{Local, NaiveDateTime};
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Phone {
    pub id: i64,
    pub company_id: Option<i64>,
    pub contact_id: Option<i64>,
    pub phone: Option<i64>,
    pub fax: bool,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

impl Phone {
    pub fn new() -> Self {
        Default::default()
    }

    fn insert(conn: &Connection, phone: Phone) -> Result<u64, String> {
        match &conn.execute(
            "
            INSERT INTO phones
            (
                company_id,
                contact_id,
                phone,
                fax,
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
                $6
            )
        ",
            &[
                &phone.company_id,
                &phone.contact_id,
                &phone.phone,
                &phone.fax,
                &Local::now().naive_local(),
                &Local::now().naive_local(),
            ],
        ) {
            Ok(1) => Ok(1),
            _ => Err("failed insert phone".to_string()),
        }
    }

    pub fn update_contacts(
        conn: &Connection,
        id: i64,
        fax: bool,
        phones: Vec<i64>,
    ) -> Result<u64, String> {
        Phone::delete_contacts(conn, id, fax);
        for value in phones {
            let mut phone = Phone::new();
            phone.contact_id = Some(id);
            phone.phone = Some(value);
            phone.fax = fax;
            Phone::insert(conn, phone)?;
        }
        Ok(0)
    }

    pub fn update_companies(
        conn: &Connection,
        id: i64,
        fax: bool,
        phones: Vec<i64>,
    ) -> Result<u64, String> {
        Phone::delete_companies(conn, id, fax);
        for value in phones {
            let mut phone = Phone::new();
            phone.company_id = Some(id);
            phone.phone = Some(value);
            phone.fax = fax;
            Phone::insert(conn, phone)?;
        }
        Ok(0)
    }

    pub fn delete_contacts(conn: &Connection, id: i64, fax: bool) {
        let _ = &conn.execute(
            "
            DELETE FROM
                phones
            WHERE
                contact_id = $1
            AND
                fax = $2
        ",
            &[&id, &fax],
        );
    }

    pub fn delete_companies(conn: &Connection, id: i64, fax: bool) {
        let _ = &conn.execute(
            "
            DELETE FROM
                phones
            WHERE
                company_id = $1
            AND
                fax = $2
        ",
            &[&id, &fax],
        );
    }
}

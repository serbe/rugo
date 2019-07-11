use chrono::{NaiveDate, NaiveDateTime};
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Contact {
    pub id: i64,
    pub name: Option<String>,
    pub company_id: Option<i64>,
    pub company_name: Option<String>,
    pub department_id: Option<i64>,
    pub department_name: Option<String>,
    pub post_id: Option<i64>,
    pub post_name: Option<String>,
    pub post_go_id: Option<i64>,
    pub post_go_name: Option<String>,
    pub rank_id: Option<i64>,
    pub rank_name: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub note: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub emails: Option<Vec<String>>,
    pub phones: Option<Vec<i64>>,
    pub faxes: Option<Vec<i64>>,
    pub educations: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
pub struct ContactList {
    pub id: i64,
    pub name: Option<String>,
    pub company_id: Option<i64>,
    pub company_name: Option<String>,
    pub post_name: Option<String>,
    pub phones: Option<Vec<i64>>,
    pub faxes: Option<Vec<i64>>,
}

#[derive(Deserialize, Serialize)]
pub struct ContactShort {
    pub id: i64,
    pub name: Option<String>,
    pub department_name: Option<String>,
    pub post_name: Option<String>,
    pub post_go_name: Option<String>,
}

impl Contact {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(conn: &Connection, id: i64) -> Result<Contact, String> {
        let mut contact = Contact::new();
        if id == 0 {
            Ok(contact)
        } else {
            for row in &conn
                .query(
                    "
                        SELECT
                            c.id,
                            c.name,
                            c.company_id,
                            co.name AS company_name,
                            c.department_id,
                            de.name AS department_name,
                            c.post_id,
                            po.name AS post_name,
                            c.post_go_id,
                            po_go.name AS post_go_name,
                            c.rank_id,
                            ra.name AS rank_name,
                            c.birthday,
                            c.note,
                            c.created_at,
                            c.updated_at,
                            array_agg(DISTINCT e.email) AS emails,
                            array_agg(DISTINCT ph.phone) AS phones,
                            array_agg(DISTINCT f.phone) AS faxes,
                            array_agg(DISTINCT ed.start_date) AS educations
                        FROM
                            contacts AS c
                        LEFT JOIN
                            companies AS co ON c.company_id = co.id
                        LEFT JOIN
                            departments AS de ON c.department_id = de.id
                        LEFT JOIN
                            posts AS po ON c.post_id = po.id
                        LEFT JOIN
                            posts AS po_go ON c.post_go_id = po_go.id
                        LEFT JOIN
                            ranks AS ra ON c.rank_id = ra.id
                        LEFT JOIN
                            emails AS e ON c.id = e.contact_id
                        LEFT JOIN
                            phones AS ph ON c.id = ph.contact_id AND ph.fax = false
                        LEFT JOIN
                            phones AS f ON c.id = f.contact_id AND f.fax = true
                        LEFT JOIN
                            educations AS ed ON c.id = ed.contact_id
                        WHERE
                            c.id = $1
                        GROUP BY
                            c.id,
                            co.name,
                            de.name,
                            po.name,
                            po_go.name,
                            ra.name
                    ",
                    &[&id],
                )
                .map_err(|e| format!("contacts id {} {}", id, e.to_string()))?
            {
                let emails = match row.get_opt(16) {
                    Some(Ok(data)) => Some(data),
                    _ => None,
                };
                let phones = match row.get_opt(17) {
                    Some(Ok(data)) => Some(data),
                    _ => None,
                };
                let faxes = match row.get_opt(18) {
                    Some(Ok(data)) => Some(data),
                    _ => None,
                };
                let educations = match row.get_opt(19) {
                    Some(Ok(data)) => Some(data),
                    _ => None,
                };
                contact = Contact {
                    id: row.get(0),
                    name: row.get(1),
                    company_id: row.get(2),
                    company_name: row.get(3),
                    department_id: row.get(4),
                    department_name: row.get(5),
                    post_id: row.get(6),
                    post_name: row.get(7),
                    post_go_id: row.get(8),
                    post_go_name: row.get(9),
                    rank_id: row.get(10),
                    rank_name: row.get(11),
                    birthday: row.get(12),
                    note: row.get(13),
                    created_at: row.get(14),
                    updated_at: row.get(15),
                    emails,
                    phones,
                    faxes,
                    educations,
                };
            }
            Ok(contact)
        }
    }
}

impl ContactList {
    pub fn get_all(conn: &Connection) -> Result<Vec<ContactList>, String> {
        let mut contacts = Vec::new();
        for row in &conn
            .query(
                "
                    SELECT
                        c.id,
                        c.name,
                        co.id AS company_id,
                        co.name AS company_name,
                        po.name AS post_name,
                        array_agg(DISTINCT ph.phone) AS phones,
                        array_agg(DISTINCT f.phone) AS faxes
                    FROM
                        contacts AS c
                    LEFT JOIN
                        companies AS co ON c.company_id = co.id
                    LEFT JOIN
                        posts AS po ON c.post_id = po.id
                    LEFT JOIN
                        phones AS ph ON c.id = ph.contact_id AND ph.fax = false
                    LEFT JOIN
                        phones AS f ON c.id = f.contact_id AND f.fax = true
                    GROUP BY
                        c.id,
                        co.id,
                        po.name
                    ORDER BY
                        name ASC
                ",
                &[],
            )
            .map_err(|e| format!("contact list {}", e.to_string()))?
        {
            let phones = match row.get_opt(5) {
                Some(Ok(data)) => Some(data),
                _ => None,
            };
            let faxes = match row.get_opt(6) {
                Some(Ok(data)) => Some(data),
                _ => None,
            };
            contacts.push(ContactList {
                id: row.get(0),
                name: row.get(1),
                company_id: row.get(2),
                company_name: row.get(3),
                post_name: row.get(4),
                phones,
                faxes,
            });
        }
        Ok(contacts)
    }
}

impl ContactShort {
    pub fn get_by_company(conn: &Connection, company_id: i64) -> Result<Vec<ContactShort>, String> {
        let mut contacts = Vec::new();
        for row in &conn
            .query(
                "
                    SELECT
                        c.id,
                        c.name,
                        d.name AS department_name,
                        p.name AS post_name,
                        pg.name AS post_go_name
                    FROM
                        contacts AS c
                    LEFT JOIN
                        departments AS d ON c.department_id = d.id
                    LEFT JOIN
                        posts AS p ON c.post_id = p.id AND p.go = false
                    LEFT JOIN
                        posts AS pg ON c.post_go_id = p.id AND p.go = true
                    WHERE
                        c.company_id = $1
                ",
                &[&company_id],
            )
            .map_err(|e| format!("contact company id {} {}", company_id, e.to_string()))?
        {
            contacts.push(ContactShort {
                id: row.get(0),
                name: row.get(1),
                department_name: row.get(2),
                post_name: row.get(3),
                post_go_name: row.get(4),
            });
        }
        Ok(contacts)
    }
}

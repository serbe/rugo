use chrono::{Local, NaiveDate, NaiveDateTime};
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Contact {
    pub id: i64,
    pub name: Option<String>,
    pub company_id: Option<i64>,
    pub department_id: Option<i64>,
    pub post_id: Option<i64>,
    pub post_go_id: Option<i64>,
    pub rank_id: Option<i64>,
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
                            c.name,
                            c.company_id,
                            c.department_id,
                            c.post_id,
                            c.post_go_id,
                            c.rank_id,
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
                            c.id
                    ",
                    &[&id],
                )
                .map_err(|e| format!("contacts id {} {}", id, e.to_string()))?
            {
                let emails = match row.get_opt(10) {
                    Some(Ok(data)) => Some(data),
                    _ => None,
                };
                let phones = match row.get_opt(11) {
                    Some(Ok(data)) => Some(data),
                    _ => None,
                };
                let faxes = match row.get_opt(12) {
                    Some(Ok(data)) => Some(data),
                    _ => None,
                };
                let educations = match row.get_opt(13) {
                    Some(Ok(data)) => Some(data),
                    _ => None,
                };
                contact = Contact {
                    id,
                    name: row.get(0),
                    company_id: row.get(1),
                    department_id: row.get(2),
                    post_id: row.get(3),
                    post_go_id: row.get(4),
                    rank_id: row.get(5),
                    birthday: row.get(6),
                    note: row.get(7),
                    created_at: row.get(8),
                    updated_at: row.get(9),
                    emails,
                    phones,
                    faxes,
                    educations,
                };
            }
            Ok(contact)
        }
    }

    pub fn post(conn: &Connection, id: i64, contact: Contact) -> Result<Contact, String> {
        if id == 0 {
            Contact::insert(conn, contact)
        } else {
            Contact::update(conn, id, contact)
        }
    }

    pub fn insert(conn: &Connection, contact: Contact) -> Result<Contact, String> {
        let mut contact = contact;
        for row in &conn
            .query(
                "
                    INSERT INTO contacts
                    (
                        name,
                        company_id,
                        department_id,
                        post_id,
                        post_go_id,
                        rank_id,
                        birthday,
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
                        $10
                    )
                    RETURNING
                        id
                ",
                &[
                    &contact.name,
                    &contact.company_id,
                    &contact.department_id,
                    &contact.post_id,
                    &contact.post_go_id,
                    &contact.rank_id,
                    &contact.birthday,
                    &contact.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .map_err(|e| format!("create contact {} ", e.to_string()))?
        {
            contact.id = row.get(0)
        }
        Ok(contact)
    }

    pub fn update(conn: &Connection, id: i64, contact: Contact) -> Result<Contact, String> {
        let mut contact = contact;
        contact.id = id;
        match &conn.execute(
            "
                UPDATE contacts SET
                    name = $2,
                    company_id = $3,
                    department_id = $4,
                    post_id = $5,
                    post_go_id = $6,
                    rank_id = $7,
                    birthday = $8,
                    note = $9,
                    updated_at = $10
                WHERE
                    id = $1
            ",
            &[
                &contact.id,
                &contact.name,
                &contact.department_id,
                &contact.post_id,
                &contact.post_go_id,
                &contact.rank_id,
                &contact.birthday,
                &contact.note,
                &Local::now().naive_local(),
            ],
        ) {
            Ok(0) => Err(format!("update contact id {}", id)),
            _ => Ok(contact),
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

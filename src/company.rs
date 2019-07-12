use chrono::NaiveDateTime;
use postgres::Connection;
use serde::{Deserialize, Serialize};

use crate::contact::ContactShort;
use crate::practice::PracticeList;

#[derive(Default, Deserialize, Serialize)]
pub struct Company {
    pub id: i64,
    pub name: Option<String>,
    pub address: Option<String>,
    pub scope_id: Option<i64>,
    pub scope_name: Option<String>,
    pub note: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub emails: Option<Vec<String>>,
    pub phones: Option<Vec<i64>>,
    pub faxes: Option<Vec<i64>>,
    pub practices: Option<Vec<PracticeList>>,
    pub contacts: Option<Vec<ContactShort>>,
}

#[derive(Deserialize, Serialize)]
pub struct CompanyList {
    pub id: i64,
    pub name: Option<String>,
    pub address: Option<String>,
    pub scope_name: Option<String>,
    pub emails: Option<Vec<String>>,
    pub phones: Option<Vec<i64>>,
    pub faxes: Option<Vec<i64>>,
    pub practices: Option<Vec<String>>,
}

impl Company {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(conn: &Connection, id: i64) -> Result<Company, String> {
        let mut company = Company::new();
        if id == 0 {
            Ok(company)
        } else {
            for row in &conn
                .query(
                    "
                        SELECT
                            c.id,
                            c.name,
                            c.address,
                            c.scope_id,
                            s.name AS scope_name,
                            c.note,
                            c.created_at,
                            c.updated_at,
                            array_agg(DISTINCT e.email) AS emails,
                            array_agg(DISTINCT ph.phone) AS phones,
                            array_agg(DISTINCT f.phone) AS faxes
                        FROM
                            companies AS c
                        LEFT JOIN
                            scopes AS s ON c.scope_id = s.id
                        LEFT JOIN
                            emails AS e ON c.id = e.company_id
                        LEFT JOIN
                            phones AS ph ON c.id = ph.company_id AND ph.fax = false
                        LEFT JOIN
                            phones AS f ON c.id = f.company_id AND f.fax = true
                        WHERE
                            c.id = $1
                        GROUP BY
                            c.id,
                            s.name
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
                let practices = PracticeList::get_by_company(conn, id).ok();
                let contacts = ContactShort::get_by_company(conn, id).ok();
                company = Company {
                    id: row.get(0),
                    name: row.get(1),
                    address: row.get(2),
                    scope_id: row.get(3),
                    scope_name: row.get(4),
                    note: row.get(5),
                    created_at: row.get(6),
                    updated_at: row.get(7),
                    emails,
                    phones,
                    faxes,
                    practices,
                    contacts,
                };
            }
            Ok(company)
        }
    }
}

impl CompanyList {
    pub fn get_all(conn: &Connection) -> Result<Vec<CompanyList>, String> {
        let mut companies = Vec::new();
        for row in &conn
            .query(
                "
					SELECT
						c.id,
						c.name,
						c.address,
						s.name AS scope_name,
						array_agg(DISTINCT e.email) AS emails,
						array_agg(DISTINCT p.phone) AS phones,
						array_agg(DISTINCT f.phone) AS faxes,
						array_agg(DISTINCT pr.date_of_practice) AS practices
					FROM
						companies AS c
					LEFT JOIN
						scopes AS s ON c.scope_id = s.id
					LEFT JOIN
						emails AS e ON c.id = e.company_id
					LEFT JOIN
						phones AS p ON c.id = p.company_id AND p.fax = false
					LEFT JOIN
						phones AS f ON c.id = f.company_id AND f.fax = true
					LEFT JOIN
						practices AS pr ON c.id = pr.company_id
					GROUP BY
						c.id,
						s.name
					ORDER BY
						c.name ASC
				",
                &[],
            )
            .map_err(|e| format!("company list {}", e.to_string()))?
        {
            let emails = match row.get_opt(4) {
                Some(Ok(data)) => Some(data),
                _ => None,
            };
            let phones = match row.get_opt(5) {
                Some(Ok(data)) => Some(data),
                _ => None,
            };
            let faxes = match row.get_opt(6) {
                Some(Ok(data)) => Some(data),
                _ => None,
            };
            let practices = match row.get_opt(7) {
                Some(Ok(data)) => Some(data),
                _ => None,
            };
            companies.push(CompanyList {
                id: row.get(0),
                name: row.get(1),
                address: row.get(2),
                scope_name: row.get(3),
                emails,
                phones,
                faxes,
                practices,
            });
        }
        Ok(companies)
    }
}
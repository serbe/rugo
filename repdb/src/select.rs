use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct SelectItem {
    pub id: i64,
    pub name: Option<String>,
}

impl SelectItem {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn company(conn: &Connection, id: i64) -> Result<SelectItem, String> {
        let mut company = SelectItem::new();
        if id == 0 {
            Ok(company)
        } else {
            for row in &conn
                .query(
                    "
						SELECT
							name
						FROM
							companies
						WHERE
							id = $1
					",
                    &[&id],
                )
                .map_err(|e| format!("company select id {} {}", id, e.to_string()))?
            {
                company.id = id;
                company.name = row.get(0);
            }
            Ok(company)
        }
    }

    pub fn company_all(conn: &Connection) -> Result<Vec<SelectItem>, String> {
        let mut companies = Vec::new();
        for row in &conn
            .query(
                "
					SELECT
						id,
						name
					FROM
						companies
					ORDER BY
						name ASC
				",
                &[],
            )
            .map_err(|e| format!("company select all {}", e.to_string()))?
        {
            companies.push(SelectItem {
                id: row.get(0),
                name: row.get(1),
            })
        }
        Ok(companies)
    }

    pub fn contact(conn: &Connection, id: i64) -> Result<SelectItem, String> {
        let mut contact = SelectItem::new();
        if id == 0 {
            Ok(contact)
        } else {
            for row in &conn
                .query(
                    "
                        SELECT
                            name
                        FROM
                            contacts
                        WHERE
                            id = $1
                    ",
                    &[&id],
                )
                .map_err(|e| format!("contact select id {} {}", id, e.to_string()))?
            {
                contact = SelectItem {
                    id,
                    name: row.get(0),
                }
            }
            Ok(contact)
        }
    }

    pub fn department(conn: &Connection, id: i64) -> Result<SelectItem, String> {
        let mut department = SelectItem::new();
        if id == 0 {
            Ok(department)
        } else {
            for row in &conn
                .query(
                    "
						SELECT
							name
						FROM
							departments
						WHERE
							id = $1
					",
                    &[&id],
                )
                .map_err(|e| format!("department select id {} {}", id, e.to_string()))?
            {
                department = SelectItem {
                    id,
                    name: row.get(0),
                };
            }
            Ok(department)
        }
    }

    pub fn department_all(conn: &Connection) -> Result<Vec<SelectItem>, String> {
        let mut departments = Vec::new();
        for row in &conn
            .query(
                "
					SELECT
						id,
						name
					FROM
						departments
					ORDER BY
						name ASC
				",
                &[],
            )
            .map_err(|e| format!("department select all {}", e.to_string()))?
        {
            departments.push(SelectItem {
                id: row.get(0),
                name: row.get(1),
            });
        }
        Ok(departments)
    }

    pub fn kind_all(conn: &Connection) -> Result<Vec<SelectItem>, String> {
        let mut kinds = Vec::new();
        for row in &conn
            .query(
                "
                    SELECT
                        id,
                        name
                    FROM
                        kinds
                    ORDER BY
                        name ASC
                ",
                &[],
            )
            .map_err(|e| format!("kind select all {}", e.to_string()))?
        {
            kinds.push(SelectItem {
                id: row.get(0),
                name: row.get(1),
            });
        }
        Ok(kinds)
    }

    pub fn post(conn: &Connection, id: i64) -> Result<SelectItem, String> {
        let mut post = SelectItem::new();
        if id == 0 {
            Ok(post)
        } else {
            for row in &conn
                .query(
                    "
                        SELECT
                            name
                        FROM
                            posts
                        WHERE
                            id = $1 AND go = false
                    ",
                    &[&id],
                )
                .map_err(|e| format!("post select id {} {}", id, e.to_string()))?
            {
                post = SelectItem {
                    id,
                    name: row.get(0),
                }
            }
            Ok(post)
        }
    }

    pub fn post_go(conn: &Connection, id: i64) -> Result<SelectItem, String> {
        let mut post = SelectItem::new();
        if id == 0 {
            Ok(post)
        } else {
            for row in &conn
                .query(
                    "
                        SELECT
                            name
                        FROM
                            posts
                        WHERE
                            id = $1 AND go = true
                    ",
                    &[&id],
                )
                .map_err(|e| format!("post_go select id {} {}", id, e.to_string()))?
            {
                post = SelectItem {
                    id,
                    name: row.get(0),
                }
            }
            Ok(post)
        }
    }

    pub fn post_all(conn: &Connection, go: bool) -> Result<Vec<SelectItem>, String> {
        let mut posts = Vec::new();
        for row in &conn
            .query(
                "
                    SELECT
                        id,
                        name
                    FROM
                        posts
                    WHERE
                        go = $1
                    ORDER BY
                        name ASC
                ",
                &[&go],
            )
            .map_err(|e| format!("post select all go {} {}", go, e.to_string()))?
        {
            posts.push(SelectItem {
                id: row.get(0),
                name: row.get(1),
            });
        }
        Ok(posts)
    }
}

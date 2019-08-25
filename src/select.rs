use postgres::Connection;
use serde::Serialize;

#[derive(Serialize)]
pub struct SelectItem {
    pub id: i64,
    pub name: Option<String>,
}

impl SelectItem {
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

    pub fn contact_all(conn: &Connection) -> Result<Vec<SelectItem>, String> {
        let mut contacts = Vec::new();
        for row in &conn
            .query(
                "
                        SELECT
                            id,
                            name
                        FROM
                            contacts
                        ORDER BY
						    name ASC
                    ",
                &[],
            )
            .map_err(|e| format!("contact select all {}", e.to_string()))?
        {
            contacts.push(SelectItem {
                id: row.get(0),
                name: row.get(1),
            })
        }
        Ok(contacts)
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

    pub fn rank_all(conn: &Connection) -> Result<Vec<SelectItem>, String> {
        let mut ranks = Vec::new();
        for row in &conn
            .query(
                "
                    SELECT
                        id,
                        name
                    FROM
                        ranks
                    ORDER BY
                        name ASC
                ",
                &[],
            )
            .map_err(|e| format!("rank select all {}", e.to_string()))?
        {
            ranks.push(SelectItem {
                id: row.get(0),
                name: row.get(1),
            });
        }
        Ok(ranks)
    }

    pub fn scope_all(conn: &Connection) -> Result<Vec<SelectItem>, String> {
        let mut scopes = Vec::new();
        for row in &conn
            .query(
                "
                    SELECT
                        id,
                        name
                    FROM
                        scopes
                    ORDER BY
                        name ASC
                ",
                &[],
            )
            .map_err(|e| format!("scope select all {}", e.to_string()))?
        {
            scopes.push(SelectItem {
                id: row.get(0),
                name: row.get(1),
            });
        }
        Ok(scopes)
    }

    pub fn siren_type_all(conn: &Connection) -> Result<Vec<SelectItem>, String> {
        let mut siren_types = Vec::new();
        for row in &conn
            .query(
                "
                    SELECT
                        id,
                        name
                    FROM
                        siren_types
                    ORDER BY
                        name ASC
                ",
                &[],
            )
            .map_err(|e| format!("siren_type select all {}", e.to_string()))?
        {
            siren_types.push(SelectItem {
                id: row.get(0),
                name: row.get(1),
            });
        }
        Ok(siren_types)
    }
}

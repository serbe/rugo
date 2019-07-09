use chrono::NaiveDateTime;
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Department {
	pub id: i64,
	pub name: Option<String>,
	pub note: Option<String>,
	pub created_at: Option<NaiveDateTime>,
	pub updated_at: Option<NaiveDateTime>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct DepartmentList {
	pub id: i64,
	pub name: Option<String>,
	pub note: Option<String>,
}

impl Department {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn get(conn: &Connection, id: i64) -> Result<Department, String> {
		let mut department = Department::new();
		if id == 0 {
			Ok(department)
		} else {
			for row in &conn
				.query(
					"
						SELECT
							id,
							name,
							note,
							created_at,
							updated_at
						FROM
							departments
						WHERE
							id = $1
					",
					&[&id],
				)
				.map_err(|e| format!("department id {} {}", id, e.to_string()))?
			{
				department = Department {
					id: row.get(0),
					name: row.get(1),
					note: row.get(2),
					created_at: row.get(3),
					updated_at: row.get(4),
				}
			}
			Ok(department)
		}
	}
}

impl DepartmentList {
	// pub fn new() -> Self {
	// 	Default::default()
	// }

	// pub fn get(conn: &Connection, id: i64) -> Result<DepartmentList, String> {
	// 	let mut department = DepartmentList::new();
	// 	if id == 0 {
	// 		Ok(department)
	// 	} else {
	// 		for row in &conn
	// 			.query(
	// 				"
	// 					SELECT
	// 						name,
	// 						note
	// 					FROM
	// 						departments
	// 					WHERE
	// 						id = $1
	// 				",
	// 				&[&id],
	// 			)
	// 			.map_err(|e| format!("department list id {} {}", id, e.to_string()))?
	// 		{
	// 			department = DepartmentList {
	// 				id,
	// 				name: row.get(0),
	// 				note: row.get(1),
	// 			};
	// 		}
	// 		Ok(department)
	// 	}
	// }

	pub fn get_all(conn: &Connection) -> Result<Vec<DepartmentList>, String> {
		let mut departments = Vec::new();
		for row in &conn
			.query(
				"
					SELECT
						id,
						name,
						note
					FROM
						departments
					ORDER BY
						name ASC
				",
				&[],
			)
			.map_err(|e| format!("department list all {}", e.to_string()))?
		{
			departments.push(DepartmentList {
				id: row.get(0),
				name: row.get(1),
				note: row.get(2),
			});
		}
		Ok(departments)
	}
}

// pub fn CreateDepartment(department Department) (int64, error) {
// 	err := e.db.Insert(&department)
// 	if err != nil {
// 		errmsg("CreateDepartment insert", err)
// 	}
// 	return department.ID, nil
// }

// pub fn UpdateDepartment(department Department) error {
// 	err := e.db.Update(&department)
// 	if err != nil {
// 		errmsg("UpdateDepartment update", err)
// 	}
// 	return err
// }

// pub fn DeleteDepartment(id int64) error {
// 	if id == 0 {
// 		return nil
// 	}
// 	else { for row in &conn.query("
// 		Where("id = ?", id).
// 		Delete()
// 	if err != nil {
// 		errmsg("DeleteDepartment delete", err)
// 	}
// 	return err
// }

// pub fn department_reateTable() error {
// 	str := `
// 		CREATE TABLE IF NOT EXISTS
// 			departments (
// 				id bigserial primary key,
// 				name text,
// 				note text,
// 				created_at TIMESTAMP without time zone,
// 				updated_at TIMESTAMP without time zone default now(),
// 				UNIQUE(name)
// 			)
// 	`
// 	_, err := e.db.Exec(str)
// 	if err != nil {
// 		errmsg("departmentCreateTable exec", err)
// 	}
// 	return err
// }

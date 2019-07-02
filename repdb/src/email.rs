use chrono::NaiveDateTime;
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Email {
	pub id: i64,
	pub company_id: Option<i64>,
	pub contact_id: Option<i64>,
	pub email: Option<String>,
	pub created_at: Option<NaiveDateTime>,
	pub updated_at: Option<NaiveDateTime>,
}

impl Email {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn get(conn: &Connection, id: i64) -> Result<Email, String> {
		let mut email = Email::new();
		if id == 0 {
			Ok(email)
		} else {
			for row in &conn
				.query(
					"
				SELECT
					company_id,
					contact_id,
					email,
					created_at,
					updated_at
				FROM
					emails
				WHERE
					id = $1
			",
					&[&id],
				)
				.map_err(|e| format!("email id {} {}", id, e.to_string()))?
			{
				email = Email {
					id,
					company_id: row.get(0),
					contact_id: row.get(1),
					email: row.get(2),
					created_at: row.get(3),
					updated_at: row.get(4),
				}
			}
			Ok(email)
		}
	}
}

// // GetEmails - get all emails for list
// pub fn GetEmails(conn: &Connection, id: i64) -> Result<Vec<Email>, String> {
// 	let mut emails = Vec::new();
// 	else { for row in &conn.query("
// 		Column("id", "email").
// 		Order("email ASC").
// 		.map_err(|e| format!(" id {} {}", id, e.to_string()))? {
// 	if err != nil {
// 		errmsg("GetEmailList select", err)
// 	}
// 	Ok(emails)
// }

// GetCompanyEmails - get all emails by company id
// pub fn get_company_emails(conn: &Connection, company_id: i64) -> Result<Vec<Email>, String> {
// 	let mut emails = Vec::new();
// 	if id == 0 {
// 		Ok(emails)
// 	}
// 	else { for row in &conn.query("
// 		Column("id", "email").
// 		Order("email ASC").
// 		Where("company_id = ?", id).
// 		.map_err(|e| format!(" id {} {}", id, e.to_string()))? {
// 	if err != nil {
// 		errmsg("GetCompanyEmails select", err)
// 	}
// 	Ok(emails)
// }

// GetContactEmails - get all emails by contact id
// pub fn GetContactEmails(conn: &Connection, id: i64) -> Result<Vec<Email>, String> {
// 	let mut emails = Vec::new();
// 	if id == 0 {
// 		Ok(emails)
// 	}
// 	else { for row in &conn.query("
// 		Column("id", "email").
// 		Order("email ASC").
// 		Where("contact_id = ?", id).
// 		.map_err(|e| format!(" id {} {}", id, e.to_string()))? {
// 	if err != nil {
// 		errmsg("GetContactEmails select", err)
// 	}
// 	Ok(emails)
// }

// CreateEmail - create new email
// pub fn CreateEmail(email Email) (int64, error) {
// 	email.ID = 0
// 	err := e.db.Insert(&email)
// 	if err != nil {
// 		errmsg("CreateEmail insert", err)
// 	}
// 	return email.ID, nil
// }

// CreateCompanyEmails - create new company email
// pub fn CreateCompanyEmails(company Company) error {
// 	err := e.DeleteCompanyEmails(company.ID)
// 	if err != nil {
// 		errmsg("CreateCompanyEmails DeleteCompanyEmails", err)
// 		return err
// 	}
// 	for i := range company.Emails {
// 		if company.Emails[i].Email != "" {
// 			company.Emails[i].CompanyID = company.ID
// 			_, err = e.CreateEmail(company.Emails[i])
// 			if err != nil {
// 				errmsg("CreateCompanyEmails CreateEmail", err)
// 				return err
// 			}
// 		}
// 	}
// 	return nil
// }

// CreateContactEmails - create new contact email
// pub fn CreateContactEmails(contact Contact) error {
// 	err := e.DeleteContactEmails(contact.ID)
// 	if err != nil {
// 		errmsg("CreateContactEmails DeleteContactEmails", err)
// 		return err
// 	}
// 	for i := range contact.Emails {
// 		if contact.Emails[i].Email != "" {
// 			contact.Emails[i].ContactID = contact.ID
// 			_, err = e.CreateEmail(contact.Emails[i])
// 			if err != nil {
// 				errmsg("CreateContactEmails CreateEmail", err)
// 				return err
// 			}
// 		}
// 	}
// 	return nil
// }

// UpdateEmail - save email changes
// pub fn UpdateEmail(email Email) error {
// 	err := e.db.Update(&email)
// 	if err != nil {
// 		errmsg("UpdateEmail update", err)
// 	}
// 	return err
// }

// DeleteEmail - delete email by id
// pub fn DeleteEmail(id int64) error {
// 	if id == 0 {
// 		return nil
// 	}
// 	else { for row in &conn.query("
// 		Where("id = ?", id).
// 		Delete()
// 	if err != nil {
// 		errmsg("DeleteEmail delete", err)
// 	}
// 	return err
// }

// DeleteCompanyEmails - delete all emails by company id
// pub fn DeleteCompanyEmails(id int64) error {
// 	if id == 0 {
// 		return nil
// 	}
// 	else { for row in &conn.query("
// 		Where("company_id = ?", id).
// 		Delete()
// 	if err != nil {
// 		errmsg("DeleteCompanyEmails delete", err)
// 	}
// 	return err
// }

// DeleteContactEmails - delete all emails by contact id
// pub fn DeleteContactEmails(id int64) error {
// 	if id == 0 {
// 		return nil
// 	}
// 	else { for row in &conn.query("
// 		Where("contact_id = ?", id).
// 		Delete()
// 	if err != nil {
// 		errmsg("DeleteContactEmails delete", err)
// 	}
// 	return err
// }

// pub fn emailCreateTable() error {
// 	str := `
// 		CREATE TABLE IF NOT EXISTS
// 			emails (
// 				id bigserial primary key,
// 				company_id bigint,
// 				contact_id bigint,
// 				email text,
// 				created_at timestamp without time zone,
// 				updated_at timestamp without time zone default now()
// 			)
// 	`
// 	_, err := e.db.Exec(str)
// 	if err != nil {
// 		errmsg("emailCreateTable exec", err)
// 	}
// 	return err
// }

use chrono::{Local, NaiveDateTime};
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Phone {
    pub id: i64,
    pub company_id: Option<i64>,
    pub contact_id: Option<i64>,
    pub phone: Option<i64>,
    pub fax: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Phone {
    pub fn new() -> Self {
        Default::default()
    }

    // pub fn get(conn: &Connection, id: i64) -> Result<Phone, String> {
    //     let mut phone = Phone::new();
    //     if id == 0 {
    //         Ok(phone)
    //     } else {
    //         for row in &conn
    //             .query(
    //                 "
    // 			SELECT
    // 				company_id,
    // 				contact_id,
    // 				phone,
    // 				fax,
    // 				create_at,
    // 				updated_at
    // 			FROM
    // 				phones
    // 			WHERE
    // 				id = $1
    // 		",
    //                 &[&id],
    //             )
    //             .map_err(|e| format!("phone id {} {}", id, e.to_string()))?
    //         {
    //             phone = Phone {
    //                 id,
    //                 company_id: row.get(0),
    //                 contact_id: row.get(1),
    //                 phone: row.get(2),
    //                 fax: row.get(3),
    //                 created_at: row.get(4),
    //                 updated_at: row.get(5),
    //             };
    //         }
    //         Ok(phone)
    //     }
    // }

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

    fn delete_contacts(conn: &Connection, id: i64, fax: bool) {
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

    fn delete_companies(conn: &Connection, id: i64, fax: bool) {
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

    //     pub fn by_company(conn: &Connection, company_id: i64, fax: bool) -> Result<Vec<Phone>, String> {
    //         let mut phones = Vec::new();
    //         if company_id == 0 {
    //             Ok(phones)
    //         } else {
    //             for row in &conn
    //                 .query(
    //                     "
    // 				SELECT
    // 					id,
    // 					company_id,
    // 					contact_id,
    // 					phone,
    // 					fax,
    // 					create_at,
    // 					updated_at
    // 				FROM
    // 					phones
    // 				WHERE
    // 					company_id = $1 AND fax = $2
    // 				ORDER BY
    // 					phone ASC
    // 			",
    //                     &[&company_id, &fax],
    //                 )
    //                 .map_err(|e| format!("phones by company {} {}", company_id, e.to_string()))?
    //             {
    //                 phones.push(Phone {
    //                     id: row.get(0),
    //                     company_id: row.get(1),
    //                     contact_id: row.get(2),
    //                     phone: row.get(3),
    //                     fax: row.get(4),
    //                     created_at: row.get(5),
    //                     updated_at: row.get(6),
    //                 });
    //             }
    //             Ok(phones)
    //         }
    //     }

    //     pub fn by_contact(conn: &Connection, contact_id: i64, fax: bool) -> Result<Vec<Phone>, String> {
    //         let mut phones = Vec::new();
    //         if contact_id == 0 {
    //             Ok(phones)
    //         } else {
    //             for row in &conn
    //                 .query(
    //                     "
    // 				SELECT
    // 					id,
    // 					company_id,
    // 					contact_id,
    // 					phone,
    // 					fax,
    // 					create_at,
    // 					updated_at
    // 				FROM
    // 					phones
    // 				WHERE
    // 					contact_id = $1 AND fax = $2
    // 				ORDER BY
    // 					phone ASC
    // 			",
    //                     &[&contact_id, &fax],
    //                 )
    //                 .map_err(|e| format!("phones by contact {} {}", contact_id, e.to_string()))?
    //             {
    //                 phones.push(Phone {
    //                     id: row.get(0),
    //                     company_id: row.get(1),
    //                     contact_id: row.get(2),
    //                     phone: row.get(3),
    //                     fax: row.get(4),
    //                     created_at: row.get(5),
    //                     updated_at: row.get(6),
    //                 });
    //             }
    //             Ok(phones)
    //         }
    //     }
}

// // // GetPhoneList - get all phones for list
// // pub fn GetPhoneList(conn: &Connection, id: i64) -> Result<Vec<Phone>, String> {
// // 	let mut phones = Vec::new();
// // 	else { for row in &conn.query("
// // 		Column("id", "company_id", "contact_id", "phone", "fax").
// // 		Order("phone ASC").
// // 		.map_err(|e| format!(" id {} {}", id, e.to_string()))? {
// // 	if err != nil {
// // 		errmsg("GetPhoneList select", err)
// // 	}
// // 	Ok(phones)
// // }

// // // CreatePhone - create new phone
// // pub fn CreatePhone(phone Phone) (int64, error) {
// // 	phone.ID = 0
// // 	err := e.db.Insert(&phone)
// // 	if err != nil {
// // 		errmsg("CreatePhone insert", err)
// // 	}
// // 	return phone.ID, nil
// // }

// // // CreateCompanyPhones - create new phones to company
// // pub fn CreateCompanyPhones(company Company, fax bool) error {
// // 	err := e.DeleteCompanyPhones(company.ID, fax)
// // 	if err != nil {
// // 		errmsg("CreateCompanyPhones DeleteCompanyPhones", err)
// // 		return err
// // 	}
// // 	let mut allPhones = Vec::new();
// // 	if fax {
// // 		allPhones = company.Faxes
// // 	} else {
// // 		allPhones = company.Phones
// // 	}
// // 	for i := range allPhones {
// // 		if allPhones[i].Phone != 0 {
// // 			let mut id = int64::new();
// // 			_ = e.db.Model(&Phone{}).
// // 				Column("id").
// // 				Where("company_id = ? and phone = ? and fax = ?", company.ID, allPhones[i].Phone, fax).
// // 				Select(&id)
// // 			if id == 0 {
// // 				allPhones[i].CompanyID = company.ID
// // 				allPhones[i].Fax = fax
// // 				_, err = e.CreatePhone(allPhones[i])
// // 				if err != nil {
// // 					errmsg("CreateCompanyPhones CreatePhone", err)
// // 					return err
// // 				}
// // 			}
// // 		}
// // 	}
// // 	return nil
// // }

// // // CreateContactPhones - create new phones to contact
// // pub fn CreateContactPhones(contact Contact, fax bool) error {
// // 	err := e.DeleteContactPhones(contact.ID, fax)
// // 	if err != nil {
// // 		errmsg("CreateContactPhones DeleteContactPhones", err)
// // 		return err
// // 	}
// // 	let mut allPhones = Vec::new();
// // 	if fax {
// // 		allPhones = contact.Faxes
// // 	} else {
// // 		allPhones = contact.Phones
// // 	}
// // 	for i := range allPhones {
// // 		if allPhones[i].Phone != 0 {
// // 			let mut id = int64::new();
// // 			_ = e.db.Model(&Phone{}).
// // 				Column("id").
// // 				Where("contact_id = ? and phone = ? and fax = ?", contact.ID, allPhones[i].Phone, fax).
// // 				Select(&id)
// // 			if id == 0 {
// // 				allPhones[i].ContactID = contact.ID
// // 				allPhones[i].Fax = fax
// // 				_, err = e.CreatePhone(allPhones[i])
// // 				if err != nil {
// // 					errmsg("CreateContactPhones CreatePhone", err)
// // 					return err
// // 				}
// // 			}
// // 		}
// // 	}
// // 	return nil
// // }

// // // DeleteCompanyPhones - delete all unnecessary phones by company id
// // pub fn DeleteCompanyPhones(id int64, fax bool) error {
// // 	if id == 0 {
// // 		return nil
// // 	}
// // 	else { for row in &conn.query("
// // 		Where("company_id = ? and fax = ?", id, fax).
// // 		Delete()
// // 	if err != nil {
// // 		errmsg("DeleteCompanyPhones delete", err)
// // 	}
// // 	return err
// // }

// // // DeleteContactPhones - delete all unnecessary phones by contact id
// // pub fn DeleteContactPhones(id int64, fax bool) error {
// // 	if id == 0 {
// // 		return nil
// // 	}
// // 	else { for row in &conn.query("
// // 		Where("contact_id = ? and fax = ?", id, fax).
// // 		Delete()
// // 	if err != nil {
// // 		errmsg("DeleteContactPhones delete", err)
// // 	}
// // 	return err
// // }

// // // DeleteAllCompanyPhones - delete all phones and faxes by company id
// // pub fn DeleteAllCompanyPhones(id int64) error {
// // 	if id == 0 {
// // 		return nil
// // 	}
// // 	else { for row in &conn.query("
// // 		Where("company_id = ?", id).
// // 		Delete()
// // 	if err != nil {
// // 		errmsg("DeleteAllCompanyPhones delete", err)
// // 	}
// // 	return err
// // }

// // // DeleteAllContactPhones - delete all phones and faxes by contact id
// // pub fn DeleteAllContactPhones(id int64) error {
// // 	if id == 0 {
// // 		return nil
// // 	}
// // 	else { for row in &conn.query("
// // 		Where("contact_id = ?", id).
// // 		Delete()
// // 	if err != nil {
// // 		errmsg("DeleteAllContactPhones delete", err)
// // 	}
// // 	return err
// // }

// // pub fn phoneCreateTable() error {
// // 	str := `
// // 		CREATE TABLE IF NOT EXISTS
// // 			phones (
// // 				id bigserial primary key,
// // 				contact_id bigint,
// // 				company_id bigint,
// // 				phone bigint,
// // 				fax bool NOT NULL DEFAULT false,
// // 				created_at TIMESTAMP without time zone,
// // 				updated_at TIMESTAMP without time zone default now()
// // 			)
// // 	`
// // 	_, err := e.db.Exec(str)
// // 	if err != nil {
// // 		errmsg("phoneCreateTable exec", err)
// // 	}
// // 	return err
// // }

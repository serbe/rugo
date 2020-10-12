use chrono::{Local, NaiveDate, NaiveDateTime};
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;

use crate::rpel::contact::ContactShort;
use crate::rpel::email::Email;
use crate::rpel::phone::Phone;
use crate::rpel::practice::PracticeList;

#[derive(Default, Deserialize, Serialize)]
pub struct Company {
    #[serde(default)]
    pub id: i64,
    pub name: Option<String>,
    pub address: Option<String>,
    pub scope_id: Option<i64>,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
    pub emails: Vec<String>,
    pub phones: Vec<i64>,
    pub faxes: Vec<i64>,
    #[serde(skip_deserializing)]
    pub practices: Vec<PracticeList>,
    #[serde(skip_deserializing)]
    pub contacts: Vec<ContactShort>,
}

#[derive(Deserialize, Serialize)]
pub struct CompanyList {
    pub id: i64,
    pub name: Option<String>,
    pub address: Option<String>,
    pub scope_name: Option<String>,
    pub emails: Vec<String>,
    pub phones: Vec<i64>,
    pub faxes: Vec<i64>,
    pub practices: Vec<NaiveDate>,
}

impl Company {
    // pub fn new() -> Self {
    //     Default::default()
    // }

    pub async fn get(client: &Client, id: i64) -> Result<Company> {
        let stmt = client
            .prepare(
                "
                    SELECT
                        c.name,
                        c.address,
                        c.scope_id,
                        c.note,
                        c.created_at,
                        c.updated_at,
                        array_remove(array_agg(e.email), NULL) AS emails,
                        array_remove(array_agg(ph.phone), NULL) AS phones,
                        array_remove(array_agg(f.phone), NULL) AS faxes
                    FROM
                        companies AS c
                    LEFT JOIN
                        emails AS e ON c.id = e.company_id
                    LEFT JOIN
                        phones AS ph ON c.id = ph.company_id AND ph.fax = false
                    LEFT JOIN
                        phones AS f ON c.id = f.company_id AND f.fax = true
                    WHERE
                        c.id = $1
                    GROUP BY
                        c.id
                ",
            )
            .await?;
        let row = client.query_one(&stmt, &[&id]).await?;
        let practices = PracticeList::get_by_company(client, id).await?;
        let contacts = ContactShort::get_by_company(client, id).await?;
        let company = Company {
            id,
            name: row.try_get(0)?,
            address: row.try_get(1)?,
            scope_id: row.try_get(2)?,
            note: row.try_get(3)?,
            created_at: row.try_get(4)?,
            updated_at: row.try_get(5)?,
            emails: row.try_get(6)?,
            phones: row.try_get(7)?,
            faxes: row.try_get(8)?,
            practices,
            contacts,
        };
        Ok(company)
    }

    pub async fn insert(client: &Client, company: Company) -> Result<Company> {
        let mut company = company;
        let stmt = client
            .prepare(
                "
                    INSERT INTO companies
                    (
                        name,
                        address,
                        scope_id,
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
                        $6
                    )
                    RETURNING
                        id
                ",
            )
            .await?;
        let row = client
            .query_one(
                &stmt,
                &[
                    &company.name,
                    &company.address,
                    &company.scope_id,
                    &company.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .await?;
        company.id = row.get(0);
        Email::update_companies(client, company.id, company.emails.clone()).await?;
        Phone::update_companies(client, company.id, false, company.phones.clone()).await?;
        Phone::update_companies(client, company.id, true, company.faxes.clone()).await?;
        Ok(company)
    }

    pub async fn update(client: &Client, company: Company) -> Result<u64> {
        let stmt = client
            .prepare(
                "
                    UPDATE companies SET
                        name = $2,
                        address = $3,
                        scope_id = $4,
                        note = $5,
                        updated_at = $6
                    WHERE
                        id = $1
                ",
            )
            .await?;
        let result = client
            .execute(
                &stmt,
                &[
                    &company.id,
                    &company.name,
                    &company.address,
                    &company.scope_id,
                    &company.note,
                    &Local::now().naive_local(),
                ],
            )
            .await?;
        Email::update_companies(client, company.id, company.emails).await?;
        Phone::update_companies(client, company.id, false, company.phones).await?;

        Phone::update_companies(client, company.id, true, company.faxes).await?;
        Ok(result)
    }

    pub async fn delete(client: &Client, id: i64) -> Result<u64> {
        Phone::delete_companies(&client, id, true).await?;
        Phone::delete_companies(&client, id, false).await?;
        Email::delete_companies(&client, id).await?;
        let stmt = client
            .prepare(
                "
                    DELETE FROM
                        companies
                    WHERE
                        id = $1
                ",
            )
            .await?;
        Ok(client.execute(&stmt, &[&id]).await?)
    }
}

impl CompanyList {
    pub async fn get_all(client: &Client) -> Result<Vec<CompanyList>> {
        let mut companies = Vec::new();
        let stmt = client
            .prepare(
                "
                    SELECT
                        c.id,
                        c.name,
                        c.address,
                        s.name AS scope_name,
                        array_remove(array_agg(e.email), NULL) AS emails,
                        array_remove(array_agg(p.phone), NULL) AS phones,
                        array_remove(array_agg(f.phone), NULL) AS faxes,
                        array_remove(array_agg(pr.date_of_practice), NULL) AS practices
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
            )
            .await?;
        for row in client.query(&stmt, &[]).await? {
            companies.push(CompanyList {
                id: row.try_get(0)?,
                name: row.try_get(1)?,
                address: row.try_get(2)?,
                scope_name: row.try_get(3)?,
                emails: row.try_get(4)?,
                phones: row.try_get(5)?,
                faxes: row.try_get(6)?,
                practices: row.try_get(7)?,
            });
        }
        Ok(companies)
    }
}

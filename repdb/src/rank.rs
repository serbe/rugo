package edc

// Rank - struct for rank
pub struct Rank {
	pub id: i64,
	pub name: Option<String>,
	pub note: Option<String>,
	pub created_at: Option<NaiveDateTime>,
	pub updated_at: Option<NaiveDateTime>,
}

// RankList - struct for rank list
pub struct RankList {
	pub id: i64,
	pub name: Option<String>,
	pub note: Option<String>,
}

// GetRank - get one rank by id
pub fn GetRank(conn: &Connection, id: i64) -> Result<Rank, String> {
	let mut rank = Rank::new();
	if id == 0 {
		Ok(rank)
	}
	else { for row in &conn.query("
		Where("id = ?", id).
		.map_err(|e| format!(" id {} {}", id, e.to_string()))? {
	if err != nil {
		errmsg("GetRank select", err)
	}
	Ok(rank)
}

// GetRankList - get rank for list by id
pub fn GetRankList(conn: &Connection, id: i64) -> Result<RankList, String> {
	let mut rank = RankList::new();
	else { for row in &conn.query("
		Column("id", "name", "note").
		Where("id = ?", id).
		Select(&rank)
	if err != nil {
		errmsg("GetRankList query", err)
	}
	Ok(rank)
}

// GetRankListAll - get all rank for list
pub fn GetRankListAll(conn: &Connection, id: i64) -> Result<Vec<RankList>, String> {
	let mut ranks = Vec::new();
	else { for row in &conn.query("
		Column("id", "name", "note").
		Order("name ASC").
		Select(&ranks)
	if err != nil {
		errmsg("GetRankListAll query", err)
	}
	Ok(ranks)
}

// GetRankSelect - get all rank for select
pub fn GetRankSelect(conn: &Connection, id: i64) -> Result<SelectItem, String> {
	let mut rank = SelectItem::new();
	if id == 0 {
		Ok(rank)
	}
	else { for row in &conn.query("
		Column("id", "name").
		Where("id = ?", id).
		Order("name ASC").
		Select(&rank)
	if err != nil {
		errmsg("GetRankSelect query", err)
	}
	Ok(rank)
}

// GetRankSelectAll - get all rank for select
pub fn GetRankSelectAll(conn: &Connection, id: i64) -> Result<Vec<SelectItem>, String> {
	let mut ranks = Vec::new();
	else { for row in &conn.query("
		Column("id", "name").
		Order("name ASC").
		Select(&ranks)
	if err != nil {
		errmsg("GetRankSelectAll query", err)
	}
	Ok(ranks)
}

// CreateRank - create new rank
pub fn CreateRank(rank Rank) (int64, error) {
	err := e.db.Insert(&rank)
	if err != nil {
		errmsg("CreateRank insert", err)
	}
	return rank.ID, err
}

// UpdateRank - save rank changes
pub fn UpdateRank(rank Rank) error {
	err := e.db.Update(&rank)
	if err != nil {
		errmsg("UpdateRank update", err)
	}
	return err
}

// DeleteRank - delete rank by id
pub fn DeleteRank(id int64) error {
	if id == 0 {
		return nil
	}
	else { for row in &conn.query("
		Where("id = ?", id).
		Delete()
	if err != nil {
		errmsg("DeleteRank delete", err)
	}
	return err
}

pub fn rankCreateTable() error {
	str := `
		CREATE TABLE IF NOT EXISTS
			ranks (
				id bigserial primary key,
				name text,
				note text,
				created_at TIMESTAMP without time zone,
				updated_at TIMESTAMP without time zone default now(),
				UNIQUE (name)
			)
	`
	_, err := e.db.Exec(str)
	if err != nil {
		errmsg("rankCreateTable exec", err)
	}
	return err
}

use chrono::{Local, NaiveDateTime};
use postgres::Connection;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Post {
    #[serde(default)]
    pub id: i64,
    pub name: Option<String>,
    pub go: bool,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct PostList {
    pub id: i64,
    pub name: Option<String>,
    pub go: bool,
    pub note: Option<String>,
}

impl Post {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(conn: &Connection, id: i64) -> Result<Post, String> {
        let mut post = Post::new();
        if id == 0 {
            Ok(post)
        } else {
            for row in &conn
                .query(
                    "
						SELECT
							name,
							go,
							note,
							created_at,
							updated_at
						FROM
							posts
						WHERE
							id = $1
					",
                    &[&id],
                )
                .map_err(|e| format!("post id {} {}", id, e.to_string()))?
            {
                post = Post {
                    id,
                    name: row.get(0),
                    go: row.get(1),
                    note: row.get(2),
                    created_at: row.get(3),
                    updated_at: row.get(4),
                }
            }
            Ok(post)
        }
    }

    pub fn insert(conn: &Connection, post: Post) -> Result<Post, String> {
        let mut post = post;
        for row in &conn
            .query(
                "
                    INSERT INTO posts
                    (
                        name,
                        go,
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
                        $5
                    )
                    RETURNING
                        id
                ",
                &[
                    &post.name,
                    &post.go,
                    &post.note,
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                ],
            )
            .map_err(|e| format!("create post {} ", e.to_string()))?
        {
            post.id = row.get(0)
        }
        Ok(post)
    }

    pub fn update(conn: &Connection, post: Post) -> Result<Post, String> {
        match &conn.execute(
            "
                UPDATE posts SET
                    name = $2,
                    go = $3,
                    note = $4,
                    updated_at = $5
                WHERE
                    id = $1
            ",
            &[
                &post.id,
                &post.name,
                &post.go,
                &post.note,
                &Local::now().naive_local(),
            ],
        ) {
            Ok(0) => Err(format!("update post id {}", post.id)),
            _ => Ok(post),
        }
    }

    pub fn delete(conn: &Connection, id: i64) -> bool {
        conn.execute(
            "
                DELETE FROM
                    posts
                WHERE
                    id = $1
            ",
            &[&id],
        )
        .is_ok()
    }
}

impl PostList {
    pub fn get_all(conn: &Connection) -> Result<Vec<PostList>, String> {
        let mut posts = Vec::new();
        for row in &conn
            .query(
                "
					SELECT
						id,
						name,
						go,
						note
					FROM
						posts
					ORDER BY
						name ASC
				",
                &[],
            )
            .map_err(|e| format!("postList all {}", e.to_string()))?
        {
            posts.push(PostList {
                id: row.get(0),
                name: row.get(1),
                go: row.get(2),
                note: row.get(3),
            });
        }
        Ok(posts)
    }
}

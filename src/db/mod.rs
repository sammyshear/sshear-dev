pub mod db {
    use std::time::{SystemTime, UNIX_EPOCH};
    use serde::{Deserialize, Serialize};

    use libsql_client::{Value, Statement, args};

    pub async fn query_posts(client: libsql_client::client::Client) -> Vec<Post> {
        let res = client.execute("SELECT id, bodyPrev, body, createdAt, title FROM posts ORDER BY createdAt DESC").await;
        let rs = res.unwrap();

        let mut posts: Vec<Post> = vec![];

        for row in rs.rows {
            let mut post: Post = Post {
                id: 0,
                title: "".into(),
                body_prev: "".into(),
                body: "".into(),
                created_at: 0,
            };
            let id_val = &row.values[0];
            match id_val {
                Value::Integer { value } => post.id = *value,
                Value::Null => post.id = 0,
                Value::Text { value: _ } => panic!(),
                Value::Blob { value: _} => panic!(),
                Value::Float { value: _} => panic!(), 
            }
            let bp_val = &row.values[1];
            match bp_val {
                Value::Integer { value: _ } => panic!(),
                Value::Null => panic!(),
                Value::Text { value } => post.body_prev = <String as Into<Box<str>>>::into(value.to_string()),
                Value::Blob { value: _} => panic!(),
                Value::Float { value: _} => panic!(),
            }
            let b_val = &row.values[2];
            match b_val {
                Value::Integer { value: _ } => panic!(),
                Value::Null => panic!(),
                Value::Text { value } => post.body = <String as Into<Box<str>>>::into(value.to_string()),
                Value::Blob { value: _} => panic!(),
                Value::Float { value: _} => panic!(),
            }
            let t_val = &row.values[4];
            match t_val {
                Value::Integer { value: _ } => panic!(),
                Value::Null => panic!(),
                Value::Text { value } => post.title = <String as Into<Box<str>>>::into(value.to_string()),
                Value::Blob { value: _} => panic!(),
                Value::Float { value: _} => panic!(),
            }
            let ca_val = &row.values[3];
            match ca_val {
                Value::Integer { value } => post.created_at = *value,
                Value::Null => panic!(),
                Value::Text { value: _ } => panic!(),
                Value::Blob { value: _} => panic!(),
                Value::Float { value: _} => panic!(), 
            }
            posts.push(post);
        }

        return posts;
    }

    pub async fn insert_post(post: PostPayload, client: libsql_client::client::Client) {
        if post.auth_key != env!("LIBSQL_CLIENT_TOKEN") { 
            return
        }
        let _ = client.execute(Statement::with_args("INSERT INTO posts (bodyPrev, body, createdAt, title) VALUES (?, ?, ?, ?)", 
            args!(post.body_prev, post.body,
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64, post.title))).await;
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Post {
        id: i64,
        pub title: Box<str>,
        pub body_prev: Box<str>,
        pub body: Box<str>,
        created_at: i64
    }
    #[derive(Deserialize, Serialize, Debug)]
    pub struct PostPayload {
        title: String,
        body_prev: String,
        body: String,
        auth_key: String
    }
}

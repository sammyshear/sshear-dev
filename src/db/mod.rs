pub mod db {
    use libsql_client::Value;

    pub async fn query_posts(client: libsql_client::client::Client) -> Vec<Post> {
        let res = client.execute("SELECT id, bodyPrev, body, createdAt FROM posts ORDER BY createdAt DESC").await;
        let rs = res.unwrap();

        let mut posts: Vec<Post> = vec![];

        for row in rs.rows {
            let mut post: Post = Post {
                id: 0,
                body_prev: "".into(),
                body: "".into(),
                created_at: 0,
            };
            let id_val = &row.values[0];
            match id_val {
                Value::Integer { value } => post.id = *value,
                Value::Null => todo!(),
                Value::Text { value: _ } => todo!(),
                Value::Blob { value: _} => todo!(),
                Value::Float { value: _} => todo!(), 
            }
            let bp_val = &row.values[1];
            match bp_val {
                Value::Integer { value: _ } => todo!(),
                Value::Null => todo!(),
                Value::Text { value } => post.body_prev = <String as Into<Box<str>>>::into(value.to_string()),
                Value::Blob { value: _} => todo!(),
                Value::Float { value: _} => todo!(),
            }
            let b_val = &row.values[2];
            match b_val {
                Value::Integer { value: _ } => todo!(),
                Value::Null => todo!(),
                Value::Text { value } => post.body = <String as Into<Box<str>>>::into(value.to_string()),
                Value::Blob { value: _} => todo!(),
                Value::Float { value: _} => todo!(),
            }
            let ca_val = &row.values[0];
            match ca_val {
                Value::Integer { value } => post.created_at = *value,
                Value::Null => todo!(),
                Value::Text { value: _ } => todo!(),
                Value::Blob { value: _} => todo!(),
                Value::Float { value: _} => todo!(), 
            }
            posts.push(post);
        }

        return posts;
    }

    #[derive(Debug)]
    pub struct Post {
        id: i64,
        body_prev: Box<str>,
        body: Box<str>,
        created_at: i64
    }
}

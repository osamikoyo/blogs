use rusqlite::{Connection, ToSql};
use serde::{Serialize};

pub struct Post{
    pub title: String,
    pub content: String,
    pub hash_tags: String,
    pub created_at: String,
}

pub struct Database{
    connection: Connection
}

pub fn database_new() -> Result<Database, rusqlite::Error>{
    let conn = Connection::open("storage/main.db")?;
    Ok(Database{connection : conn})
}

impl Database{
    pub fn create_tables(&self) -> Result<(), Err()>{
        self.connection.execute(
            "create table if not exists posts( \
            id integer primary key,\
            title text not null,\
            content text not null,\
            hash_tags text,\
        )",
            ()
        )?;

        Ok(())
    }

    pub fn post_add(&self, post: Post) -> Result<(), Err()>{
        self.connection.execute("insert into posts (title, content, hash_tags, \
        created_at) values (?1, ?2, ?3, ?4)", (&post.title, &post.content, &post.hash_tags, &post.created_at),
        )?;
    }

    pub fn post_get(&self, title : String) -> Vec<Post> {
        let mut request = self.connection.prepare(
            "select id, title, content, hash_tags from posts where title=:title;"
        ).unwrap();

        let posts_iter = request.query_map(&[(":title", title.to_string().as_str())], |row| {
                Ok(Post {
                    title: row.get(0)?,
                    content: row.get(1)?,
                    hash_tags: row.get(2)?,
                    created_at: row.get(3)?,
                })
        }).unwrap();
        let mut posts : Vec<Post> = Vec::new();
        for p in posts_iter{
            match p {
                Ok(post) => posts.push(post),
                Err(e)=>
                    println!("Error fetching post - {}", e)
            }
        }

        posts
    }

    pub fn post_update(&self, post: Post) -> Result<(), Err()> {
        self.connection.execute(
            "update posts set content = ?1, hash_tags = ?2 where title = ?3",
            &[&post.content, &post.hash_tags, &post.title]
        )?;

        
    }
}

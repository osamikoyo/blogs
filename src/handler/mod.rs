mod data;

use actix_web::{get, web, post, Responder, HttpResponse};
use crate::handler::data::{database_new, Post};

#[get("/")]
async fn hello() -> impl Responder{
    "helloo brou"
}

#[post("/api/post/add")]
async fn add_post(post : web::Json<Post>) -> impl Responder{
    let data = database_new()?;

    let post_iter = Post{
        title: post.title.clone(),
        content: post.content.clone(),
        hash_tags: post.hash_tags.clone(),
        created_at: chrono::Utc::now().to_string(),
    };

    data.post_add(post_iter)?;

    match data {
        Ok(c) => "success! - ".to_string().push_str(c),
        Err(e) => "Error! - ".to_string().push_str(e)
    }
}

#[get("/api/post/get/{title}")]
async fn get_post(title : web::Path<String>) -> impl Responder{
    let real_title = title.to_string();

    let db = database_new()?;

    let posts : Vec<Post> = db.post_get(real_title);

    Ok(web::Json(posts))
}

#[get("/api/post/update/")]
async fn update_post(data : web::Path<Post>) -> impl Responder{
    let real_data = Post{
        title: data.title.clone(),
        content: data.content.clone(),
        hash_tags: data.hash_tags.clone(),
        created_at: "".to_string(),
    };
    let db = database_new()?;

    db.post_update(real_data)?;

    match db{
        Ok(()) => "success!",
        Err(e) => "Error = ".to_string().push_str(e)
    }
}




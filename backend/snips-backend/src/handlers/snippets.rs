use crate::models::snippets::CreateSnippet;
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/snippets/{id}")]
async fn get_snippet(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder
{
    "get_snippet"
}

#[post("/users/{user_id}/snippets")]
async fn create_snippet(user_id: web::Path<i32>, snip: web::Json<CreateSnippet>, db_pool: web::Data<PgPool>) -> impl Responder
{
    "create_snippet"
}
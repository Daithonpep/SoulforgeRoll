// src/api/souls.rs
use actix_web::{web, HttpResponse, Responder, Result};
use uuid::Uuid;
use sqlx::PgPool;
use crate::souls::{
    AgingSystem, DeathSystem, DeathEvent,
    lineage::{LineageSystem, HeirCreation},
    echo::{EchoSystem, EchoContext},
};
use serde::{Deserialize, Serialize};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/souls")
            .route("/{id}/session-end", web::post().to(end_session))
            .route("/{id}/death", web::post().to(process_death))
            .route("/{id}/retire", web::post().to(retire))
            .route("/{id}/create-heir", web::post().to(create_heir))
            .route("/{id}/check-echo", web::post().to(check_echo))
            .route("/{id}/trigger-echo", web::post().to(dm_trigger_echo))
    );
    cfg.route("/wall-of-fallen", web::get().to(get_fallen));
}

async fn end_session(
    pool: web::Data<PgPool>,
    soul_id: web::Path<Uuid>,
) -> Result<impl Responder> {
    let result = AgingSystem::process_session_aging(pool.get_ref(), *soul_id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    
    if result.triggered_event.as_deref() == Some("natural_death") {
        DeathSystem::process_death(
            pool.get_ref(),
            *soul_id,
            DeathEvent {
                cause: "Muerte natural por vejez".to_string(),
                location: "En paz".to_string(),
                killed_by: None,
                last_words: Some("Mi tiempo ha llegado...".to_string()),
                witnesses: vec![],
            },
        ).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    }
    
    Ok(HttpResponse::Ok().json(result))
}

async fn process_death(
    pool: web::Data<PgPool>,
    soul_id: web::Path<Uuid>,
    death: web::Json<DeathEvent>,
) -> Result<impl Responder> {
    let result = DeathSystem::process_death(pool.get_ref(), *soul_id, death.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(result))
}

#[derive(Deserialize)]
struct RetireRequest {
    location: String,
    title: String,
}

async fn retire(
    pool: web::Data<PgPool>,
    soul_id: web::Path<Uuid>,
    req: web::Json<RetireRequest>,
) -> Result<impl Responder> {
    DeathSystem::process_retirement(pool.get_ref(), *soul_id, &req.location, &req.title).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}

async fn create_heir(
    pool: web::Data<PgPool>,
    _id: web::Path<Uuid>,
    heir_data: web::Json<HeirCreation>,
) -> Result<impl Responder> {
    let (new_id, traits) = LineageSystem::create_heir(pool.get_ref(), heir_data.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "heir_id": new_id,
        "inherited": traits
    })))
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct FallenHero {
    id: Uuid,
    name: String,
    title: Option<String>,
    age_years: Option<i32>,
    death_cause: Option<String>,
    death_location: Option<String>,
    death_date: Option<chrono::NaiveDateTime>, 
    kills_total: Option<i32>,
    sessions_played: Option<i32>,
}

async fn get_fallen(
    pool: web::Data<PgPool>,
) -> Result<impl Responder> {
    let fallen: Vec<FallenHero> = sqlx::query_as(
        r#"
        SELECT id, name, title, age_years, death_cause, death_location, 
               death_date, kills_total, sessions_played
        FROM wall_of_fallen 
        LIMIT 50
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(fallen))
}

#[derive(Deserialize)] // EchoContext needs Deserialize to be used in web::Json
pub struct EchoContextDTO { // Use DTO to avoid conflict or just use EchoContext
    // EchoContext in souls/echo.rs derives Deserialize.
    // Let's reuse that.
}

async fn check_echo(
    pool: web::Data<PgPool>,
    heir_id: web::Path<Uuid>,
    context: web::Json<EchoContext>,
) -> Result<impl Responder> {
    let echo = EchoSystem::check_echo_triggers(pool.get_ref(), *heir_id, &context).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(echo))
}

#[derive(Deserialize)]
struct TriggerRequest {
    ancestor_id: Uuid,
    message: Option<String>
}

async fn dm_trigger_echo(
    pool: web::Data<PgPool>,
    heir_id: web::Path<Uuid>,
    req: web::Json<TriggerRequest>,
) -> Result<impl Responder> {
    let echo = EchoSystem::dm_trigger_echo(pool.get_ref(), *heir_id, req.ancestor_id, req.message.clone()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(echo))
}

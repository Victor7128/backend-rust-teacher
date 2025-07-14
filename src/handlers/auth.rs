use actix_web::{get, post, web, HttpResponse, Responder};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, RedirectUrl, TokenUrl};
use serde::Deserialize;
use sqlx::PgPool;
use crate::models::{Usuario, ErrorResponse};

// Configurar cliente OAuth de Google
pub fn google_oauth_client() -> BasicClient {
    BasicClient::new(
        ClientId::new(std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set")),
        None,
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
        Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap())
    )
    .set_redirect_uri(
        RedirectUrl::new(
            std::env::var("GOOGLE_REDIRECT_URI").expect("GOOGLE_REDIRECT_URI must be set")
        ).unwrap()
    )
}

#[derive(Deserialize)]
pub struct GoogleCallbackQuery {
    code: String,
}

#[get("/login/google")]
pub async fn login_google() -> HttpResponse {
    let client = google_oauth_client();
    let (auth_url, _csrf_token) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("email".to_string()))
        .add_scope(oauth2::Scope::new("profile".to_string()))
        .url();
    
    HttpResponse::Found()
        .append_header(("Location", auth_url.to_string()))
        .finish()
}

#[get("/auth/google/callback")]
pub async fn google_callback(
    db: web::Data<PgPool>,
    query: web::Query<GoogleCallbackQuery>,
) -> Result<HttpResponse, ErrorResponse> {
    let token = google_oauth_client()
        .exchange_code(oauth2::AuthorizationCode::new(query.code.clone()))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(|e| ErrorResponse::internal(e.to_string()))?;
    
    // Obtener información del usuario
    let client = reqwest::Client::new();
    let user_info = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .map_err(|e| ErrorResponse::internal(e.to_string()))?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| ErrorResponse::internal(e.to_string()))?;
    
    let email = user_info["email"]
        .as_str()
        .ok_or_else(|| ErrorResponse::bad_request("Email not found"))?
        .to_string();
    
    let name = user_info["name"]
        .as_str()
        .ok_or_else(|| ErrorResponse::bad_request("Name not found"))?
        .to_string();
    
    let google_id = user_info["sub"]
        .as_str()
        .ok_or_else(|| ErrorResponse::bad_request("Google ID not found"))?
        .to_string();
    
    // Crear o actualizar usuario
    let usuario = sqlx::query_as!(
        Usuario,
        r#"
        INSERT INTO usuarios (id, email, nombre, google_id)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (google_id) DO UPDATE
        SET email = EXCLUDED.email, nombre = EXCLUDED.nombre
        RETURNING *
        "#,
        uuid::Uuid::new_v4(),
        email,
        name,
        google_id
    )
    .fetch_one(db.get_ref())
    .await
    .map_err(|e| ErrorResponse::internal(e.to_string()))?;
    
    // TODO: Generar JWT y devolverlo
    Ok(HttpResponse::Ok().json(usuario))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login_google)
        .service(google_callback);
}
use actix_web::{web, HttpResponse, HttpRequest, Responder, Error};
use crate::utils::data::*;
use crate::utils::token::*;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(|| async { HttpResponse::Ok().body("Server is working") })),
    )
    .service(web::resource("/api/login").route(web::post().to(login)))
    .service(web::resource("/api/logout").route(web::post().to(logout)))
    .service(web::resource("/api/try_luck").route(web::post().to(try_luck_user)));
}

async fn login(req: web::Json<LoginRequest>) -> Result<impl Responder, Error> {
    match generate_new_token(req.into_inner()) {
        Some(random_token) => {
            if add_token(random_token.clone().as_str()).await {
                Ok(HttpResponse::Ok().json(LoginResponse { token: random_token }))
            } else {
                Ok(HttpResponse::InternalServerError().body("Error saving token"))
            }
        }
        None => Ok(HttpResponse::Unauthorized().json(ErrorResponse { error: "Wrong email and password".to_string() })),
    }
}


async fn logout(req: HttpRequest) -> Result<impl Responder, Error> {
    let user_token = get_user_token(&req);
    if remove_token(&user_token).await {
    Ok(HttpResponse::Ok().body("\"OK\""))
    } else {
        Ok(HttpResponse::InternalServerError().body("Error removing token"))
    }
}

async fn try_luck_user(req: HttpRequest) -> Result<impl Responder, Error> {
    let user_token = get_user_token(&req);
    let total_wins = get_total_wins().await.unwrap_or(0);
    let did_win = if total_wins > 30 { rand::random::<f32>() < 0.4 } else { rand::random::<f32>() < 0.7 };
    
    if update_win(&user_token, did_win).await {
        Ok(HttpResponse::Ok().json(WinnerResponse { win: did_win }))
    } else {
        Ok(HttpResponse::InternalServerError().body("Error trying"))
    }
}
use doxa_auth::{error::UserNotFound, guard::AuthGuard};
use doxa_core::{actix_web::web, error::HttpResponse, EndpointResult};
use doxa_db::PgPool;

use self::response::PublicUserInfo;

pub mod response;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/user/info", web::post().to(current_user_info))
        .route("/user/info/{username}", web::get().to(info));
}

async fn info(db_pool: web::Data<PgPool>, username: web::Path<String>) -> EndpointResult {
    let conn = web::block({
        let db_pool = db_pool.clone();
        move || db_pool.get()
    })
    .await??;

    let username = username.into_inner();
    let user = web::block(move || doxa_db::action::user::get_user_by_username(&conn, &username))
        .await??
        .ok_or(UserNotFound)?;

    let conn = web::block(move || db_pool.get()).await??;
    let competitions =
        web::block(move || doxa_db::action::competition::list_user_enrollments(&conn, user.id))
            .await??
            .into_iter()
            .map(|c| c.name)
            .collect();

    Ok(HttpResponse::Ok().json(PublicUserInfo {
        username: user.username,
        admin: user.admin,
        competitions,
        extra: user.extra,
    }))
}

async fn current_user_info(db_pool: web::Data<PgPool>, user: AuthGuard<()>) -> EndpointResult {
    let conn = web::block({
        let db_pool = db_pool.clone();
        move || db_pool.get()
    })
    .await??;

    let user_id = user.id();
    let user = web::block(move || doxa_db::action::user::get_user_by_id(&conn, user_id)).await??;

    let conn = web::block(move || db_pool.get()).await??;
    let competitions =
        web::block(move || doxa_db::action::competition::list_user_enrollments(&conn, user_id))
            .await??
            .into_iter()
            .map(|c| c.name)
            .collect();

    Ok(HttpResponse::Ok().json(PublicUserInfo {
        username: user.username,
        admin: user.admin,
        competitions,
        extra: user.extra,
    }))
}

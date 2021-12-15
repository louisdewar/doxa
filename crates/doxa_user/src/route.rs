use doxa_auth::guard::AuthGuard;
use doxa_core::{actix_web::web, error::HttpResponse, EndpointResult};
use doxa_db::PgPool;

use self::response::UserInfo;

mod response;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/user/info", web::post().to(info));
}

async fn info(db_pool: web::Data<PgPool>, user: AuthGuard<()>) -> EndpointResult {
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

    Ok(HttpResponse::Ok().json(UserInfo {
        username: user.username,
        admin: user.admin,
        competitions,
    }))
}

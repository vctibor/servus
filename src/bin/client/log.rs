use servus::persistence::*;
use actix_web::{web, Error, HttpResponse, HttpRequest};
use servus::DbPool;

pub async fn get_log_entries(req: HttpRequest, pool: web::Data<DbPool>)
                    -> Result<HttpResponse, Error>
{
    let offset: i64 = req.match_info().get("offset").ok_or_else(|| {
        eprintln!("Missing offset parameter.");
        HttpResponse::InternalServerError().finish()
    })?.parse().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let entries: i64 = req.match_info().get("entries").ok_or_else(|| {
        eprintln!("Missing offset parameter.");
        HttpResponse::InternalServerError().finish()
    })?.parse().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    println!("Get {} log entries with offset {}.", entries, offset);

    let conn = pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    let log_entries = web::block(move || log::get_log(offset, entries, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    println!("{:?}", log_entries);

    Ok(HttpResponse::Ok().json(log_entries))
}
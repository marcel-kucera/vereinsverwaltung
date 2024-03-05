use axum::{extract::Query, http::StatusCode, Json};
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::AppState;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Member {
    id: i32,
    firstname: String,
    lastname: String,
    email: String,
    plz: String,
    city: String,
    street: String,
    housenumber: String,
    iban: String,
    bic: String,
    sepa: bool,
    note: Option<String>,
    joindate: i32,
    paid: bool,
}

#[derive(Deserialize, Serialize)]
pub struct MemberNew {
    firstname: String,
    lastname: String,
    email: String,
    plz: String,
    city: String,
    street: String,
    housenumber: String,
    iban: String,
    bic: String,
    sepa: bool,
    note: Option<String>,
    joindate: i32,
}

#[derive(Deserialize, Serialize)]
pub struct MemberIdQuery {
    id: i32,
}

pub async fn get_member(
    state: AppState,
    id: Option<Query<MemberIdQuery>>,
) -> Result<Json<Vec<Member>>, StatusCode> {
    let currentyear = chrono::Utc::now().year();

    let res = sqlx::query_as::<_, Member>(&format!(
        "select
        m.*,
        {} in (select p.year from payment as p where memberid = m.id) as paid
        from member as m
        {}
        order by firstname asc",
        currentyear,
        id.as_ref()
            .map(|v| format!("where m.id = {}", v.id))
            .unwrap_or("".to_string())
    ))
    .fetch_all(&state.db)
    .await
    .unwrap();

    if id.is_some() && res.len() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(res))
}

pub async fn post_member(state: AppState, n: Json<MemberNew>) -> StatusCode {
    sqlx::query(
        "
                   insert into member 
                   (firstname,
                    lastname,
                    email,
                    plz,
                    city,
                    street,
                    housenumber,
                    iban,
                    bic,
                    sepa,
                    note,
                    joindate
                   )
                   values (?,?,?,?,?,?,?,?,?,?,?,?)",
    )
    .bind(&n.firstname)
    .bind(&n.lastname)
    .bind(&n.email)
    .bind(&n.plz)
    .bind(&n.city)
    .bind(&n.street)
    .bind(&n.housenumber)
    .bind(&n.iban)
    .bind(&n.bic)
    .bind(&n.sepa)
    .bind(&n.note)
    .bind(&n.joindate)
    .execute(&state.db)
    .await
    .unwrap();

    StatusCode::OK
}

pub async fn delete_member(state: AppState, q: Query<MemberIdQuery>) {
    sqlx::query("delete from member where id = ?")
        .bind(q.id)
        .execute(&state.db)
        .await
        .unwrap();
}

pub async fn put_member(state: AppState, q: Query<MemberIdQuery>, n: Json<MemberNew>) {
    sqlx::query(
        "update member set
                   firstname=?,
                    lastname=?,
                    email=?,
                    plz=?,
                    city=?,
                    street=?,
                    housenumber=?,
                    iban=?,
                    bic=?,
                    sepa=?,
                    note=?,
                    joindate=?,
                    note=?
        where id=?
                ",
    )
    .bind(&n.firstname)
    .bind(&n.lastname)
    .bind(&n.email)
    .bind(&n.plz)
    .bind(&n.city)
    .bind(&n.street)
    .bind(&n.housenumber)
    .bind(&n.iban)
    .bind(&n.bic)
    .bind(&n.sepa)
    .bind(&n.note)
    .bind(&n.joindate)
    .bind(&n.note)
    .bind(q.id)
    .execute(&state.db)
    .await
    .unwrap();
}

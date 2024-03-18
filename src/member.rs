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
    birthday: i32,
    fee: f32,
    mandate: String,
    deleted: bool,
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
    birthday: i32,
    fee: f32,
    mandate: String,
}

#[derive(Deserialize, Serialize)]
pub struct MemberIdQuery {
    id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct ShowDeletedQuery {
    show_deleted: bool,
}

pub async fn get_member(
    state: AppState,
    id: Option<Query<MemberIdQuery>>,
    show_deleted: Option<Query<ShowDeletedQuery>>,
) -> Result<Json<Vec<Member>>, StatusCode> {
    let current_year = chrono::Utc::now().year();

    let where_clause = if let Some(ref id_query) = id {
        format!("where m.id = {}", id_query.id)
    } else {
        format!(
            "where m.deleted = {}",
            show_deleted.map_or("0", |f| if f.show_deleted { "1" } else { "0" })
        )
    };

    let res = sqlx::query_as::<_, Member>(&format!(
        "select
        m.*,
        {current_year} in (select p.year from payment as p where memberid = m.id) as paid
        from member as m
        {where_clause}
        order by firstname asc"
    ))
    .fetch_all(&state.db)
    .await
    .unwrap();

    if id.is_some() && res.is_empty() {
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
                    joindate,
                    birthday,
                    fee,
                    mandate
                   )
                   values (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
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
    .bind(n.sepa)
    .bind(&n.note)
    .bind(n.joindate)
    .bind(n.birthday)
    .bind(n.fee)
    .bind(&n.mandate)
    .execute(&state.db)
    .await
    .unwrap();

    StatusCode::OK
}

pub async fn delete_member(state: AppState, q: Query<MemberIdQuery>) {
    sqlx::query("update member set deleted=1 where id = ?")
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
                    note=?,
                    birthday=?,
                    fee=?,
                    mandate=?
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
    .bind(n.sepa)
    .bind(&n.note)
    .bind(n.joindate)
    .bind(&n.note)
    .bind(n.birthday)
    .bind(n.fee)
    .bind(&n.mandate)
    .bind(q.id)
    .execute(&state.db)
    .await
    .unwrap();
}

pub async fn restore_member(state: AppState, q: Query<MemberIdQuery>) {
    sqlx::query("update member set deleted=0 where id=?")
        .bind(q.id)
        .execute(&state.db)
        .await
        .unwrap();
}

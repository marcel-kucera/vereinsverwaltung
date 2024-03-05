use axum::{extract::Query, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::AppState;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Payment {
    id: i32,
    year: i32,
    memberid: i32,
}

#[derive(Deserialize, Serialize)]
pub struct PaymentNew {
    year: i32,
    memberid: i32,
}

#[derive(Deserialize, Serialize)]
pub struct PaymentMemberIdQuery {
    memberid: i32,
}

pub async fn get_payment(s: AppState, q: Query<PaymentMemberIdQuery>) -> Json<Vec<Payment>> {
    Json(
        sqlx::query_as::<_, Payment>("select * from payment where memberid = ? order by year desc")
            .bind(q.0.memberid)
            .fetch_all(&s.db)
            .await
            .unwrap(),
    )
}

pub async fn post_payment(s: AppState, p: Json<PaymentNew>) -> StatusCode {
    let res = sqlx::query("insert into payment (year,memberid) values (?,?)")
        .bind(p.year)
        .bind(p.memberid)
        .execute(&s.db)
        .await;
    match res {
        Ok(_) => StatusCode::OK,
        Err(e) => e
            .into_database_error()
            .map(|e| {
                if e.is_unique_violation() {
                    StatusCode::BAD_REQUEST
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            })
            .unwrap(),
    }
}

#[derive(Deserialize, Serialize)]
pub struct PaymentIdQuery {
    id: i32,
}

pub async fn delete_payment(state: AppState, n: Query<PaymentIdQuery>) {
    sqlx::query("delete from payment where id = ?")
        .bind(n.id)
        .execute(&state.db)
        .await
        .unwrap();
}

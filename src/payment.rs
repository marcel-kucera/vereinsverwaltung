use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{
    error::{AppError, UserError},
    AppState,
};

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

pub async fn get_payment(
    s: AppState,
    q: Query<PaymentMemberIdQuery>,
) -> Result<Json<Vec<Payment>>, AppError> {
    let payments =
        sqlx::query_as::<_, Payment>("select * from payment where memberid = ? order by year desc")
            .bind(q.0.memberid)
            .fetch_all(&s.db)
            .await?;
    Ok(Json(payments))
}

pub async fn post_payment(s: AppState, p: Json<PaymentNew>) -> Result<(), AppError> {
    let res = sqlx::query("insert into payment (year,memberid) values (?,?)")
        .bind(p.year)
        .bind(p.memberid)
        .execute(&s.db)
        .await;

    if let Err(e) = res {
        match e.as_database_error() {
            Some(db_error) => {
                if db_error.is_unique_violation() {
                    return Err(UserError::DuplicateError.into());
                }
            }
            None => return Err(e.into()),
        }
    }

    Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct PaymentIdQuery {
    id: i32,
}

pub async fn delete_payment(state: AppState, n: Query<PaymentIdQuery>) -> Result<(), AppError> {
    sqlx::query("delete from payment where id = ?")
        .bind(n.id)
        .execute(&state.db)
        .await?;
    Ok(())
}

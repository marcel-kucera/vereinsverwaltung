use axum::{body::Bytes, extract::Query, http::header, response::IntoResponse, Json};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{
    error::{AppError, UserError},
    AppState,
};

#[derive(Deserialize, Serialize, FromRow)]
pub struct FileEntry {
    id: i32,
    name: String,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct File {
    id: i32,
    memberid: i32,
    name: String,
    file: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
pub struct MemberIdQuery {
    memberid: i32,
}

pub async fn get_filelist(
    query: Query<MemberIdQuery>,
    state: AppState,
) -> Result<Json<Vec<FileEntry>>, AppError> {
    let res = sqlx::query_as::<_, FileEntry>("select id,name from memberfile where memberid = ?")
        .bind(query.memberid)
        .fetch_all(&state.db)
        .await?;
    Ok(Json(res))
}

#[derive(Deserialize, Serialize)]
pub struct IdQuery {
    id: i32,
}

pub async fn get_file(
    state: AppState,
    query: Query<IdQuery>,
) -> Result<impl IntoResponse, AppError> {
    let res = sqlx::query_as::<_, File>("select * from memberfile where id = ?")
        .bind(query.id)
        .fetch_one(&state.db)
        .await?;

    Ok((
        [(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename={}", res.name),
        )],
        res.file,
    ))
}

#[derive(TryFromMultipart)]
pub struct FileUpload {
    memberid: i32,
    #[form_data(limit = "1GiB")]
    file: FieldData<Bytes>,
}

pub async fn post_file(
    state: AppState,
    upload: TypedMultipart<FileUpload>,
) -> Result<(), AppError> {
    let form = upload.0;

    let memberid = form.memberid;
    let filename = form
        .file
        .metadata
        .file_name
        .ok_or(UserError::FileNameMissingError)?;
    let file = form.file.contents.to_vec();

    sqlx::query("insert into memberfile (memberid,name,file) values (?,?,?)")
        .bind(memberid)
        .bind(filename)
        .bind(file)
        .execute(&state.db)
        .await?;
    Ok(())
}

pub async fn delete_file(state: AppState, q: Query<IdQuery>) -> Result<(), AppError> {
    sqlx::query("delete from memberfile where id = ?")
        .bind(q.id)
        .execute(&state.db)
        .await?;
    Ok(())
}

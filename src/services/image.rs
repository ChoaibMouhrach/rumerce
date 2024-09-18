use std::path::Path;

use sqlx::{postgres::PgQueryResult, Error, PgConnection};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use uuid::Uuid;

use crate::{models::image::Image, validations::image::StoreImageSchema};

pub async fn all(connection: &mut PgConnection) -> Result<Vec<Image>, Error> {
    sqlx::query_as!(Image, "SELECT * FROM images")
        .fetch_all(connection)
        .await
}

pub async fn find(id: &Uuid, connection: &mut PgConnection) -> Result<Option<Image>, Error> {
    sqlx::query_as!(Image, "SELECT * FROM images WHERE id = $1", id)
        .fetch_optional(connection)
        .await
}

pub async fn find_by_src(src: &str, connection: &mut PgConnection) -> Result<Option<Image>, Error> {
    sqlx::query_as!(Image, "SELECT * FROM images WHERE src = $1", src)
        .fetch_optional(connection)
        .await
}

pub async fn insert(
    input: &StoreImageSchema,
    connection: &mut PgConnection,
) -> Result<Image, Error> {
    sqlx::query_as!(
        Image,
        "INSERT INTO images(name, src) VALUES ($1, $2) RETURNING *",
        input.name,
        input.src
    )
    .fetch_one(connection)
    .await
}

pub async fn upload(
    input: &StoreImageSchema,
    bytes: &[u8],
    connection: &mut PgConnection,
) -> Result<Image, Box<dyn std::error::Error>> {
    let public_path = Path::new("public");

    if let Err(err) = fs::create_dir_all(&public_path).await {
        return Err(err.into());
    };

    let file_path = public_path.join(&input.src);

    let mut file = match File::create(file_path).await {
        Ok(file) => file,
        Err(err) => {
            return Err(err.into());
        }
    };

    if let Err(err) = file.write_all(bytes).await {
        return Err(err.into());
    }

    match insert(input, connection).await {
        Ok(image) => Ok(image),
        Err(err) => Err(err.into()),
    }
}

pub async fn update(
    id: &Uuid,
    input: &StoreImageSchema,
    connection: &mut PgConnection,
) -> Result<PgQueryResult, Error> {
    sqlx::query!(
        "UPDATE images SET name = $2, src = $3 WHERE id = $1",
        id,
        input.name,
        input.src
    )
    .execute(connection)
    .await
}

pub async fn destroy(id: &Uuid, connection: &mut PgConnection) -> Result<PgQueryResult, Error> {
    sqlx::query!("DELETE FROM images WHERE id = $1", id,)
        .execute(connection)
        .await
}

pub async fn unload(image: &Image, connection: &mut PgConnection) -> Result<PgQueryResult, Error> {
    let public_path = Path::new("public");

    if let Err(err) = fs::create_dir_all(&public_path).await {
        return Err(err.into());
    };

    let file_path = public_path.join(&image.src);

    if let Err(err) = fs::remove_file(file_path).await {
        return Err(err.into());
    }

    match destroy(&image.id, connection).await {
        Ok(query_result) => Ok(query_result),
        Err(err) => Err(err.into()),
    }
}

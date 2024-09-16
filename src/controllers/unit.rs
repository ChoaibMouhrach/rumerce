use crate::{services, validations::unit::StoreUnitSchema, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use log::error;
use uuid::Uuid;

pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let units = match services::unit::all(&mut connection).await {
        Ok(units) => units,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    Json(units).into_response()
}

pub async fn show(Path(id): Path<Uuid>, State(state): State<AppState>) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let unit = match services::unit::find(&id, &mut connection).await {
        Ok(Some(unit)) => unit,
        Ok(None) => {
            return (StatusCode::NOT_FOUND).into_response();
        }
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    Json(unit).into_response()
}

pub async fn store(
    State(state): State<AppState>,
    Json(input): Json<StoreUnitSchema>,
) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let new_unit = services::unit::find_by_name(&input.name, &mut connection).await;

    if let Err(err) = new_unit {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    if let Ok(Some(_)) = new_unit {
        return (StatusCode::FORBIDDEN).into_response();
    }

    if let Err(err) = services::unit::insert(&input, &mut connection).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    (StatusCode::CREATED).into_response()
}

pub async fn update(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(input): Json<StoreUnitSchema>,
) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let unit = match services::unit::find(&id, &mut connection).await {
        Ok(Some(unit)) => unit,
        Ok(None) => {
            return (StatusCode::NOT_FOUND).into_response();
        }
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let new_unit = services::unit::find_by_name(&input.name, &mut connection).await;

    if let Err(err) = new_unit {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    if let Ok(Some(new_unit)) = new_unit {
        if new_unit.id != unit.id {
            return (StatusCode::FORBIDDEN).into_response();
        }
    }

    if let Err(err) = services::unit::update(&id, &input, &mut connection).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    (StatusCode::NO_CONTENT).into_response()
}

pub async fn destroy(Path(id): Path<Uuid>, State(state): State<AppState>) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    if let Err(err) = services::unit::destroy(&id, &mut connection).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    (StatusCode::NO_CONTENT).into_response()
}

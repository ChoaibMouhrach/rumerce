use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use biscotti::{Processor, ProcessorConfig, RequestCookies};
use log::error;
use uuid::Uuid;

use crate::{
    models::session::PopulatedSession,
    services,
    validations::cart::{StoreCartItemSchema, StoreCartSchema},
    AppState,
};

pub async fn add_item(
    headers: HeaderMap,
    State(state): State<AppState>,
    Extension(auth): Extension<Option<PopulatedSession>>,
    Json(input): Json<StoreCartItemSchema>,
) -> impl IntoResponse {
    let cart_id = headers
        .get(header::COOKIE)
        .and_then(|cookie_header| cookie_header.to_str().ok())
        .and_then(|cookies| {
            let processor: Processor = ProcessorConfig::default().into();
            RequestCookies::parse_header(cookies, &processor).ok()
        })
        .and_then(|cookies| cookies.get("cart_id"))
        .and_then(|cookie| Some(cookie.value().to_string()))
        .and_then(|cart_id| Uuid::parse_str(&cart_id).ok());

    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let cart = match cart_id {
        Some(cart_id) => match services::cart::find(&cart_id, &mut connection).await {
            Ok(cart) => cart,
            Err(err) => {
                error!("{err}");
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        },
        None => None,
    };

    let cart = match cart {
        Some(cart) => cart,
        None => {
            let input = StoreCartSchema {
                user_id: match auth {
                    Some(auth) => Some(auth.user.id),
                    None => None,
                },
            };

            let cart = match services::cart::insert(&input, &mut connection).await {
                Ok(cart) => cart,
                Err(err) => {
                    error!("{err}");
                    return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                }
            };

            cart
        }
    };

    if let Err(err) = services::cart::add_item(&cart.cart.id, &input, &mut connection).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    ().into_response()
}

pub async fn list_items(
    headers: HeaderMap,
    State(state): State<AppState>,
    Extension(auth): Extension<Option<PopulatedSession>>,
) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let cart = match auth {
        Some(auth) => match services::cart::find_by_user_id(&auth.user.id, &mut connection).await {
            Ok(Some(cart)) => Some(cart),
            Ok(None) => None,
            Err(err) => {
                error!("{err}");
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        },
        None => None,
    };

    let cart = match cart {
        Some(cart) => Some(cart),
        None => {
            let cart_id = headers
                .get(header::COOKIE)
                .and_then(|cookie_header| cookie_header.to_str().ok())
                .and_then(|cookies| {
                    let processor: Processor = ProcessorConfig::default().into();
                    RequestCookies::parse_header(cookies, &processor).ok()
                })
                .and_then(|cookies| cookies.get("cart_id"))
                .and_then(|cookie| Some(cookie.value().to_string()))
                .and_then(|cart_id| Uuid::parse_str(&cart_id).ok());

            match cart_id {
                Some(cart_id) => match services::cart::find(&cart_id, &mut connection).await {
                    Ok(Some(cart)) => Some(cart),
                    Ok(None) => None,
                    Err(err) => {
                        error!("{err}");
                        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                    }
                },
                None => None,
            }
        }
    };

    let items = match cart {
        Some(cart) => cart.items,
        None => Vec::new(),
    };

    (Json(items)).into_response()
}

pub async fn delete_item(
    Path(item_id): Path<Uuid>,
    headers: HeaderMap,
    Extension(auth): Extension<Option<PopulatedSession>>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let cart = match auth {
        Some(auth) => match services::cart::find_by_user_id(&auth.user.id, &mut connection).await {
            Ok(Some(cart)) => Some(cart),
            Ok(None) => None,
            Err(err) => {
                error!("{err}");
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        },
        None => None,
    };

    let cart = match cart {
        Some(cart) => Some(cart),
        None => {
            let cart_id = headers
                .get(header::COOKIE)
                .and_then(|cookie_header| cookie_header.to_str().ok())
                .and_then(|cookies| {
                    let processor: Processor = ProcessorConfig::default().into();
                    RequestCookies::parse_header(cookies, &processor).ok()
                })
                .and_then(|cookies| cookies.get("cart_id"))
                .and_then(|cookie| Some(cookie.value().to_string()))
                .and_then(|cart_id| Uuid::parse_str(&cart_id).ok());

            match cart_id {
                Some(cart_id) => match services::cart::find(&cart_id, &mut connection).await {
                    Ok(Some(cart)) => Some(cart),
                    Ok(None) => None,
                    Err(err) => {
                        error!("{err}");
                        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                    }
                },
                None => None,
            }
        }
    };

    let cart = match cart {
        Some(cart) => cart,
        None => {
            return ().into_response();
        }
    };

    if let Err(err) = services::cart::delete_item(&cart.cart.id, &item_id, &mut connection).await {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    ().into_response()
}

pub async fn update_item(
    Path(item_id): Path<Uuid>,
    headers: HeaderMap,
    Extension(auth): Extension<Option<PopulatedSession>>,
    State(state): State<AppState>,
    Json(input): Json<StoreCartItemSchema>,
) -> impl IntoResponse {
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(err) => {
            error!("{err}");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let cart = match auth {
        Some(auth) => match services::cart::find_by_user_id(&auth.user.id, &mut connection).await {
            Ok(Some(cart)) => Some(cart),
            Ok(None) => None,
            Err(err) => {
                error!("{err}");
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        },
        None => None,
    };

    let cart = match cart {
        Some(cart) => Some(cart),
        None => {
            let cart_id = headers
                .get(header::COOKIE)
                .and_then(|cookie_header| cookie_header.to_str().ok())
                .and_then(|cookies| {
                    let processor: Processor = ProcessorConfig::default().into();
                    RequestCookies::parse_header(cookies, &processor).ok()
                })
                .and_then(|cookies| cookies.get("cart_id"))
                .and_then(|cookie| Some(cookie.value().to_string()))
                .and_then(|cart_id| Uuid::parse_str(&cart_id).ok());

            match cart_id {
                Some(cart_id) => match services::cart::find(&cart_id, &mut connection).await {
                    Ok(Some(cart)) => Some(cart),
                    Ok(None) => None,
                    Err(err) => {
                        error!("{err}");
                        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                    }
                },
                None => None,
            }
        }
    };

    let cart = match cart {
        Some(cart) => cart,
        None => {
            return ().into_response();
        }
    };

    if let Err(err) =
        services::cart::update_item(&cart.cart.id, &item_id, &input, &mut connection).await
    {
        error!("{err}");
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    ().into_response()
}

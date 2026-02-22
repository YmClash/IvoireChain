//! Custom error types for IvoireChain.
//!
//! This module defines a hierarchy of errors following Clean Architecture:
//! - `DomainError`: Business logic violations
//! - `InfrastructureError`: Technical failures (DB, cache, blockchain)
//! - `ApplicationError`: Use case execution failures
//! - `ApiError`: HTTP-friendly error responses

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

/// Domain layer errors - business rule violations
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Solde insuffisant: requis {required} FCFA, disponible {available} FCFA")]
    InsufficientBalance { required: i64, available: i64 },

    #[error("Numéro de téléphone invalide: {0}")]
    InvalidPhoneNumber(String),

    #[error("Numéros de ticket invalides: {0}")]
    InvalidTicketNumbers(String),

    #[error("Montant invalide: {0}")]
    InvalidAmount(String),

    #[error("Utilisateur non trouvé")]
    UserNotFound,

    #[error("Ticket non trouvé")]
    TicketNotFound,

    #[error("Tirage non trouvé")]
    DrawNotFound,

    #[error("Le tirage est fermé aux participations")]
    DrawClosed,

    #[error("Utilisateur déjà existant avec ce numéro")]
    UserAlreadyExists,
}

/// Infrastructure layer errors - technical failures
#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("Erreur base de données: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Erreur cache Redis: {0}")]
    Cache(String),

    #[error("Erreur blockchain: {0}")]
    Blockchain(String),

    #[error("Erreur de connexion: {0}")]
    Connection(String),

    #[error("Timeout: {0}")]
    Timeout(String),
}

/// Application layer errors - use case failures
#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error(transparent)]
    Infrastructure(#[from] InfrastructureError),

    #[error("Authentification échouée: {0}")]
    AuthenticationFailed(String),

    #[error("Token invalide ou expiré")]
    InvalidToken,

    #[error("Accès non autorisé")]
    Unauthorized,

    #[error("Validation échouée: {0}")]
    ValidationFailed(String),
}

/// API error response structure
#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    pub error: String,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

/// HTTP API errors with status codes
#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    Application(#[from] ApplicationError),

    #[error("Requête invalide: {0}")]
    BadRequest(String),

    #[error("Ressource non trouvée: {0}")]
    NotFound(String),

    #[error("Erreur interne du serveur")]
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message, details) = match &self {
            ApiError::Application(app_err) => match app_err {
                ApplicationError::Domain(domain_err) => match domain_err {
                    DomainError::InsufficientBalance { .. } => (
                        StatusCode::BAD_REQUEST,
                        "INSUFFICIENT_BALANCE",
                        domain_err.to_string(),
                        None,
                    ),
                    DomainError::InvalidPhoneNumber(_) => (
                        StatusCode::BAD_REQUEST,
                        "INVALID_PHONE",
                        domain_err.to_string(),
                        None,
                    ),
                    DomainError::InvalidTicketNumbers(_) => (
                        StatusCode::BAD_REQUEST,
                        "INVALID_NUMBERS",
                        domain_err.to_string(),
                        None,
                    ),
                    DomainError::UserNotFound
                    | DomainError::TicketNotFound
                    | DomainError::DrawNotFound => (
                        StatusCode::NOT_FOUND,
                        "NOT_FOUND",
                        domain_err.to_string(),
                        None,
                    ),
                    DomainError::UserAlreadyExists => (
                        StatusCode::CONFLICT,
                        "USER_EXISTS",
                        domain_err.to_string(),
                        None,
                    ),
                    DomainError::DrawClosed => (
                        StatusCode::BAD_REQUEST,
                        "DRAW_CLOSED",
                        domain_err.to_string(),
                        None,
                    ),
                    _ => (
                        StatusCode::BAD_REQUEST,
                        "DOMAIN_ERROR",
                        domain_err.to_string(),
                        None,
                    ),
                },
                ApplicationError::AuthenticationFailed(_) | ApplicationError::InvalidToken => (
                    StatusCode::UNAUTHORIZED,
                    "AUTH_FAILED",
                    app_err.to_string(),
                    None,
                ),
                ApplicationError::Unauthorized => (
                    StatusCode::FORBIDDEN,
                    "FORBIDDEN",
                    app_err.to_string(),
                    None,
                ),
                ApplicationError::ValidationFailed(_) => (
                    StatusCode::BAD_REQUEST,
                    "VALIDATION_ERROR",
                    app_err.to_string(),
                    None,
                ),
                ApplicationError::Infrastructure(infra_err) => {
                    tracing::error!("Infrastructure error: {:?}", infra_err);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "INTERNAL_ERROR",
                        "Une erreur technique s'est produite".to_string(),
                        None,
                    )
                }
            },
            ApiError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone(), None)
            }
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone(), None),
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "Une erreur interne s'est produite".to_string(),
                None,
            ),
        };

        let body = Json(ApiErrorResponse {
            error: message,
            code: code.to_string(),
            details,
        });

        (status, body).into_response()
    }
}

/// Result type alias for application layer
pub type AppResult<T> = Result<T, ApplicationError>;

/// Result type alias for API handlers
pub type ApiResult<T> = Result<T, ApiError>;

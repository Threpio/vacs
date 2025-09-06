use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;

pub mod error;
pub mod session;

pub enum MaybeJsonOrProblem<T> {
    Json(T),
    NoContent,
    Problem(error::ProblemDetails),
}

impl<T> MaybeJsonOrProblem<T> {
    pub fn ok(value: T) -> Self {
        Self::Json(value)
    }
    pub fn no_content() -> Self {
        Self::NoContent
    }
    pub fn problem(problem: error::ProblemDetails) -> Self {
        Self::Problem(problem)
    }
}

impl<T: Serialize> IntoResponse for MaybeJsonOrProblem<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            MaybeJsonOrProblem::Json(json) => (StatusCode::OK, axum::Json(json)).into_response(),
            MaybeJsonOrProblem::NoContent => StatusCode::NO_CONTENT.into_response(),
            MaybeJsonOrProblem::Problem(problem) => problem.into_response(),
        }
    }
}

pub type ApiResult<T> = Result<axum::Json<T>, error::AppError>;
pub type ApiMaybe<T> = Result<MaybeJsonOrProblem<T>, error::AppError>;
pub type StatusCodeResult = Result<StatusCode, error::AppError>;

use actix_web::{get, http::StatusCode, web, HttpResponse, ResponseError};

pub struct Name(pub String);

#[derive(serde::Deserialize)]
pub struct QueryParams {
    name: String,
}

impl Name {
    pub fn parse(name: String) -> Result<Name, String> {
        if name.trim().is_empty() {
            Err("Name can not be empty".to_string())
        } else {
            Ok(Self(name))
        }
    }
}

impl TryFrom<QueryParams> for Name {
    type Error = String;

    fn try_from(value: QueryParams) -> Result<Self, Self::Error> {
        let name = Name::parse(value.name)?;
        Ok(name)
    }
}

#[derive(thiserror::Error)]
pub enum ExampleError {
    #[error("{0}")]
    ValidationError(String),
    #[error("Something went horribly wrong")]
    UnexpectedError(#[from] anyhow::Error),
}
impl ResponseError for ExampleError {
    fn status_code(&self) -> StatusCode {
        match self {
            ExampleError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ExampleError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl std::fmt::Debug for ExampleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
impl From<String> for ExampleError {
    fn from(e: String) -> Self {
        Self::ValidationError(e)
    }
}

#[get("/example")]
#[tracing::instrument(
    name = "example endpoint triggered",
    skip(query),
    fields(
        query = %query.name,
    )
)]
async fn example(query: web::Query<QueryParams>) -> Result<HttpResponse, ExampleError> {
    // whichever works
    let name: Name = query.0.try_into()?;
    //let name = Name::try_from(query.0)?;

    Ok(HttpResponse::Ok().body(name.0))
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

use std::fmt;

use conduit::Response;

use super::{AppError, Bad, StringError};
use crate::util::json_response;

#[derive(Debug)]
pub(super) struct Ok(pub(super) String);
#[derive(Debug)]
pub(super) struct ServerError(pub(super) String);

impl AppError for Ok {
    fn response(&self) -> Option<Response> {
        Some(json_response(&Bad {
            errors: vec![StringError {
                detail: self.0.clone(),
            }],
        }))
    }
}

impl fmt::Display for Ok {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AppError for ServerError {
    fn response(&self) -> Option<Response> {
        let mut response = json_response(&Bad {
            errors: vec![StringError {
                detail: self.0.clone(),
            }],
        });
        response.status = (500, "Internal Server Error");
        Some(response)
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

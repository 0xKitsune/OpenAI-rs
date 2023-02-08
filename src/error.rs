use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenAIError {
    #[error("Reqwest Error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Your account is not active, please check your billing details on our website.")]
    BillingNotActive,
    #[error("Unrecognized error")]
    UnrecognizedError(String),
}
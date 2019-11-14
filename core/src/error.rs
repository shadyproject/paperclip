#[cfg(feature = "v2")]
use crate::v2::models::{DataType, ParameterIn};
use failure::Fail;

/// Errors related to spec validation.
#[derive(Debug, Fail)]
pub enum ValidationError {
    /// Failed to resolve the schema because an invalid URI was provided for
    /// `$ref` field.
    ///
    /// Currently, we only support `#/{definitions,parameters}/Name` in `$ref` field.
    #[fail(
        display = "Invalid $ref URI {:?}. Only relative URIs are supported.",
        _0
    )]
    InvalidRefURI(String),
    /// The specified reference is missing in the spec.
    #[fail(display = "Reference missing in spec: {}", _0)]
    MissingReference(String),
    /// If a parameter specifies body, then schema must be specified.
    #[fail(
        display = "Parameter {:?} in path {:?} is a body but the schema is missing",
        _0, _1
    )]
    MissingSchemaForBodyParameter(String, String),
    /// Some headers have special meaning in OpenAPI. The user cannot have these headers
    /// in their API spec.
    #[fail(
        display = "Path {:?} has header parameter {:?} which is not allowed",
        _1, _0
    )]
    InvalidHeader(String, String),
    #[cfg(feature = "v2")]
    /// Only arrays and primitive types are allowed in parameters.
    #[fail(
        display = "Parameter {:?} in path {:?} has specified {:?} type, but it's invalid for {:?} parameters",
        _0, _1, _2, _3
    )]
    InvalidParameterType(String, String, Option<DataType>, ParameterIn),
}

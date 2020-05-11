pub use bcrypt::BcryptError;
pub use diesel::result::Error as DieselError;
pub use jsonwebtoken::errors::{Error as JWTError, ErrorKind as JWTErrorKind};
pub use std::error::Error as StdError;
pub use std::fmt::{Display as FmtDisplay, Formatter as FmtFormatter, Result as FmtResult};
pub use std::option::NoneError;
pub use std::result::Result as StdResult;

pub type Consequence<T> = StdResult<T, Error>;

// ----------------------------------------------------------------------------------- USER ERRORS

#[derive(Debug)]
pub enum EntityError {
    Duplicate,
    NotIdentifiable,
    EmptyAttribute,
    InvalidID,
    InvalidAttribute,
}

impl FmtDisplay for EntityError {
    fn fmt(&self, f: &mut FmtFormatter) -> FmtResult {
        match self {
            EntityError::Duplicate => write!(f, "Entity already exist"),
            EntityError::NotIdentifiable => write!(f, "The entity do not have an ID"),
            EntityError::EmptyAttribute => {
                write!(f, "At least one of the entity's attributes is empty")
            }
            EntityError::InvalidID => {
                write!(f, "The given identifier does not correspond to any entity")
            }
            EntityError::InvalidAttribute => {
                write!(f, "At least one of the given attributes is invalid")
            }
        }
    }
}

impl StdError for EntityError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

// ---------------------------------------------------------------------------------- POSTS ERRORS

#[derive(Debug)]
pub enum PostError {
    UnknownKind,
    InvalidKind,
    InvalidAnswer,
}

impl FmtDisplay for PostError {
    fn fmt(&self, f: &mut FmtFormatter) -> FmtResult {
        match self {
            PostError::UnknownKind => write!(f, "Unknown post kind"),
            PostError::InvalidKind => {
                write!(f, "The post kind is invalid for the requested operation")
            }
            PostError::InvalidAnswer => write!(f, "This answer is not linked to this post"),
        }
    }
}

impl StdError for PostError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

// ---------------------------------------------------------------------------------- TOKEN ERRORS

#[derive(Debug)]
pub enum TokenError {
    Consumed,
    Expired,
    Collision, // This error type should not happen often
    InvalidHash,
}

impl FmtDisplay for TokenError {
    fn fmt(&self, f: &mut FmtFormatter) -> FmtResult {
        match self {
            TokenError::Consumed => write!(f, "This token has already been consumed"),
            TokenError::Expired => write!(f, "This token has expired"),
            TokenError::Collision => write!(f, "Token hash collision occurred"),
            TokenError::InvalidHash => write!(f, "Invalid token hash"),
        }
    }
}

impl StdError for TokenError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

// ----------------------------------------------------------------------------------- USER ERRORS

#[derive(Debug)]
pub enum UserError {
    InvalidEmail,
}

impl FmtDisplay for UserError {
    fn fmt(&self, f: &mut FmtFormatter) -> FmtResult {
        match self {
            UserError::InvalidEmail => write!(f, "Only UNamur staff/students may register"),
        }
    }
}

impl StdError for UserError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

// ----------------------------------------------------------------------------------- AUTH ERRORS

#[derive(Debug)]
pub enum AuthError {
    InvalidIDs,
    Inactive,
    AlreadyActivated,
    InvalidToken,
    MissingHeader,
    InvalidHeader,
    MissingCapability,
}

impl FmtDisplay for AuthError {
    fn fmt(&self, f: &mut FmtFormatter) -> FmtResult {
        match self {
            AuthError::InvalidIDs => write!(f, "Unable to login with this email/password"),
            AuthError::Inactive => write!(f, "Account needs activation"),
            AuthError::AlreadyActivated => write!(f, "Account was already activated"),
            AuthError::InvalidToken => write!(f, "Invalid token"),
            AuthError::MissingHeader => write!(f, "Missing header"),
            AuthError::InvalidHeader => write!(f, "Invalid header"),
            AuthError::MissingCapability => write!(f, "Missing capability"),
        }
    }
}

impl StdError for AuthError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

// ----------------------------------------------------------------------------- WATCH EVENT ERRORS


#[derive(Debug)]
pub enum WatchEventError {
    UnknownKind,
    InvalidKind,
    InvalidWatchTransition
}

impl FmtDisplay for WatchEventError {
    fn fmt(&self, f: &mut FmtFormatter) -> FmtResult {
        match self {
            WatchEventError::UnknownKind => write!(f, "Unknown watch event kind"),
            WatchEventError::InvalidKind => {
                write!(f, "The watch event is invalid for the requested operation")
            },
            WatchEventError::InvalidWatchTransition => {
                write!(f, "Previous watch event cannot lead to the requested event")
            }
        }
    }
}

impl StdError for WatchEventError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

// ---------------------------------------------------------------------------------- MODEL ERRORS

#[derive(Debug)]
pub enum Error {
    NotFound,
    DatabaseError(DieselError),
    BCryptError(BcryptError),
    TokenError(TokenError),
    JWTError(JWTError),
    UserError(UserError),
    EntityError(EntityError),
    AuthError(AuthError),
    PostError(PostError),
    WatchEventError(WatchEventError)
}

impl FmtDisplay for Error {
    fn fmt(&self, f: &mut FmtFormatter) -> FmtResult {
        match self {
            Error::NotFound => write!(f, "The seeked record(s) could not be found"),
            Error::DatabaseError(e) => write!(f, "{}", e),
            Error::BCryptError(e) => write!(f, "{}", e),
            Error::EntityError(e) => write!(f, "{}", e),
            Error::TokenError(e) => write!(f, "{}", e),
            Error::JWTError(e) => write!(f, "{}", e),
            Error::UserError(e) => write!(f, "{}", e),
            Error::AuthError(e) => write!(f, "{}", e),
            Error::PostError(e) => write!(f, "{}", e),
            Error::WatchEventError(e) => write!(f, "{}", e)
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::DatabaseError(ref e) => Some(e),
            Error::BCryptError(ref e) => Some(e),
            Error::TokenError(ref e) => Some(e),
            Error::JWTError(ref e) => Some(e),
            Error::UserError(ref e) => Some(e),
            Error::EntityError(ref e) => Some(e),
            Error::AuthError(ref e) => Some(e),
            Error::PostError(ref e) => Some(e),
            Error::WatchEventError(ref e ) => Some(e),
            _ => None,
        }
    }
}

// ----------------------------------------------------------------------------------- CONVERSIONS

// () -> Error
impl From<NoneError> for Error {
    fn from(_: NoneError) -> Error {
        Error::NotFound
    }
}

// DieselError -> Error
impl From<DieselError> for Error {
    fn from(error: DieselError) -> Error {
        match error {
            DieselError::NotFound => Error::NotFound,
            other => Error::DatabaseError(other),
        }
    }
}

// BcryptError -> Error
impl From<BcryptError> for Error {
    fn from(error: BcryptError) -> Error {
        Error::BCryptError(error)
    }
}

// EntityError -> Error
impl From<EntityError> for Error {
    fn from(error: EntityError) -> Error {
        Error::EntityError(error)
    }
}

// TokenError -> Error
impl From<TokenError> for Error {
    fn from(error: TokenError) -> Error {
        Error::TokenError(error)
    }
}

// JWTError -> Error
impl From<JWTError> for Error {
    fn from(error: JWTError) -> Error {
        Error::JWTError(error)
    }
}

// UserError -> Error
impl From<UserError> for Error {
    fn from(error: UserError) -> Error {
        Error::UserError(error)
    }
}

// AuthError -> Error
impl From<AuthError> for Error {
    fn from(error: AuthError) -> Error {
        Error::AuthError(error)
    }
}

// PostError -> Error
impl From<PostError> for Error {
    fn from(error: PostError) -> Error {
        Error::PostError(error)
    }
}

impl From<WatchEventError> for Error {
    fn from(error: WatchEventError) -> Error { Error::WatchEventError(error) }
}
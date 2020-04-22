pub use bcrypt::BcryptError;
pub use diesel::result::Error as DieselError;
pub use jsonwebtoken::errors::{Error as JWTError, ErrorKind as JWTErrorKind};
pub use std::error::Error as StdError;
use std::fmt::{Display as FmtDisplay, Formatter as FmtFormatter, Result as FmtResult};
pub use std::option::NoneError;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

pub type EntityError = entity::Error;
pub type TokenError = token::Error;
pub type UserError = user::Error;
pub type AuthError = auth::Error;

// ----------------------------------------------------------------------------------- USER ERRORS

pub mod entity {
    use std::error::Error as StdError;
    use std::fmt::{Display as FmtDisplay, Formatter as FmtFormatter, Result as FmtResult};

    #[derive(Debug)]
    pub enum Error {
        Duplicate,
    }

    impl FmtDisplay for Error {
        fn fmt(&self, f: &mut FmtFormatter) -> FmtResult {
            match self {
                Error::Duplicate => write!(f, "Entity already exist"),
            }
        }
    }

    impl StdError for Error {
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            None
        }
    }
}

// ---------------------------------------------------------------------------------- TOKEN ERRORS

pub mod token {
    use std::error::Error as StdError;
    use std::fmt::{Display as FmtDisplay, Formatter as FmtFormatter, Result as FmtResult};

    #[derive(Debug)]
    pub enum Error {
        Consumed,
        Expired,
        Collision, // This error type should not happen often
        InvalidHash,
    }

    impl FmtDisplay for Error {
        fn fmt(&self, f: &mut FmtFormatter) -> FmtResult {
            match self {
                Error::Consumed => write!(f, "This token has already been consumed"),
                Error::Expired => write!(f, "This token has expired"),
                Error::Collision => write!(f, "Token hash collision occured"),
                Error::InvalidHash => write!(f, "Invalid token hash"),
            }
        }
    }

    impl StdError for Error {
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            None
        }
    }
}

// ----------------------------------------------------------------------------------- USER ERRORS

pub mod user {
    use std::error::Error as StdError;
    use std::fmt::{Display as FmtDisplay, Formatter as FmtFormatter, Result as FmtResult};

    #[derive(Debug)]
    pub enum Error {
        InvalidEmail,
    }

    impl FmtDisplay for Error {
        fn fmt(&self, f: &mut FmtFormatter) -> FmtResult {
            match self {
                Error::InvalidEmail => write!(f, "Only UNamur staff/students may register"),
            }
        }
    }

    impl StdError for Error {
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            None
        }
    }
}

// ----------------------------------------------------------------------------------- AUTH ERRORS

pub mod auth {
    use std::error::Error as StdError;
    use std::fmt::{Display as FmtDisplay, Formatter as FmtFormatter, Result as FmtResult};

    #[derive(Debug)]
    pub enum Error {
        InvalidIDs,
        Inactive,
        AlreadyActivated,
        InvalidToken,
        MissingHeader,
        InvalidHeader,
    }

    impl FmtDisplay for Error {
        fn fmt(&self, f: &mut FmtFormatter) -> FmtResult {
            match self {
                Error::InvalidIDs => write!(f, "Unable to login with this email/password"),
                Error::Inactive => write!(f, "Account needs activation"),
                Error::AlreadyActivated => write!(f, "Account was already activated"),
                Error::InvalidToken => write!(f, "Invalid token"),
                Error::MissingHeader => write!(f, "Missing header"),
                Error::InvalidHeader => write!(f, "Invalid header"),
            }
        }
    }

    impl StdError for Error {
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            None
        }
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

use nom::{error::VerboseError, IResult};

pub type Res<T, U> = IResult<T, U, VerboseError<T>>;

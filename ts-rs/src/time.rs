use time::{
    Date, Duration, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset
};

use super::{impl_primitives, TS};
use crate::Dependency;

impl_primitives!(Date, Duration, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset => "string");
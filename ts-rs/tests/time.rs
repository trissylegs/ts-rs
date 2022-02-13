#![cfg(feature = "time-impl")]
#![allow(unused)]

use ts_rs::TS;
use time::{
    Date, Duration, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset
};

#[test]
fn time() {

    #[derive(TS)]
    struct TimeImpl {
        date: Date, 
        duration: Duration,
        offset_date_time: OffsetDateTime,
        primitive_date_time: PrimitiveDateTime,
        time: Time,
        utc_offset: UtcOffset,
    }
    assert_eq!(TimeImpl::decl(), "interface TimeImpl { date: string, duration: string, offset_date_time: string, primitive_date_time: string, time: string, utc_offset: string, }")
}

#![allow(dead_code)]

use crate::deserialize::FromSqlRow;
use crate::expression::AsExpression;
use std::time::SystemTime;

#[derive(AsExpression, FromSqlRow)]
#[diesel(foreign_derive)]
#[diesel(sql_type = crate::sql_types::Timestamp)]
struct SystemTimeProxy(SystemTime);

#[cfg(feature = "chrono")]
mod chrono {
    extern crate chrono;
    use self::chrono::*;
    use crate::deserialize::FromSqlRow;
    use crate::expression::AsExpression;
    use crate::sql_types::{Date, Time, Timestamp};

    #[derive(AsExpression, FromSqlRow)]
    #[diesel(foreign_derive)]
    #[diesel(sql_type = Date)]
    struct NaiveDateProxy(NaiveDate);

    #[derive(AsExpression, FromSqlRow)]
    #[diesel(foreign_derive)]
    #[diesel(sql_type = Time)]
    struct NaiveTimeProxy(NaiveTime);

    #[derive(AsExpression, FromSqlRow)]
    #[diesel(foreign_derive)]
    #[diesel(sql_type = Timestamp)]
    #[cfg_attr(
        feature = "postgres_backend",
        diesel(sql_type = crate::sql_types::Timestamptz)
    )]
    #[cfg_attr(feature = "mysql_backend", diesel(sql_type = crate::sql_types::Datetime))]
    struct NaiveDateTimeProxy(NaiveDateTime);

    #[derive(FromSqlRow)]
    #[cfg_attr(feature = "postgres_backend", derive(AsExpression))]
    #[diesel(foreign_derive)]
    #[cfg_attr(
        feature = "postgres_backend",
        diesel(sql_type = crate::sql_types::Timestamptz)
    )]
    struct DateTimeProxy<Tz: TimeZone>(DateTime<Tz>);
}

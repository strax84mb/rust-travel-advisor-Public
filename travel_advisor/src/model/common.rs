use csv::StringRecord;

use crate::util::app_errors::Error;

pub trait FromStringRecord {
    type Output;

    #[must_use]
    fn from_string_record(record: StringRecord) -> Result<Self::Output, Error>;
}
use csv::StringRecord;

use crate::util::Error;

pub trait FromStringRecord {
    type Output;

    #[must_use]
    fn from_string_record(record: StringRecord) -> Result<Self::Output, Error>;
}
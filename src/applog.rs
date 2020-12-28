use chrono::prelude::*;


struct AppLog<'d> {
     timestamp: DateTime<Utc>,
     entry: &'d str
 }
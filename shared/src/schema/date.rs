use  chrono::DateTime;// this might be useful
const current_timezone: String = web_sys::Storage::local_storage().unwrap().get_item("timezone").unwrap().unwrap();

struct Date {
    string,
}


impl Date {
    fn new(string: String) -> Self {
        Self { string }
    }
}

impl From<String> for Date {
    fn from(date: string) -> Self {
        date.convet_to_utc_timzone(from:current_timezone)
    }
}

impl To<String> for Date {
    fn to(date: string) -> Self {
        date.convet_current_user_timezone(to:current_timezone)
    }
}
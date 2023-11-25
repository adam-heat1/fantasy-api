use std::env;

pub(crate) const ERROR: String =
    env::var("NTFY_UNKNOWN_ERROR").expect("NTFY_UNKNOWN_ERROR must be set");
pub(crate) const MEDIA: String =
    env::var("NTFY_UNKNOWN_MEDIA").expect("NTFY_UNKNOWN_MEDIA must be set");

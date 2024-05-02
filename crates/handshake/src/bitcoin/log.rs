use std::fmt::Debug;
use tracing::info;

pub fn info_in_message<M: Debug>(msg: &M) {
    info!(direction = "IN", message = format!("{:#?}", msg));
}

pub fn info_out_message<M: Debug>(msg: &M) {
    info!(direction = "OUT", message = format!("{:#?}", msg));
}

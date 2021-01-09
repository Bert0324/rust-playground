mod logger;
use logger::logger::controller::log;
mod data;
use data::data::database::get_data;
use data::source::source::source::get_source;
mod config;

fn main() {
    let mut v = get_data();
    v.push_str(&get_source());
    log(v);
}

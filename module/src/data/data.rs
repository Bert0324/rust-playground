pub mod database {
    use crate::config::config;
    pub fn get_data() -> String {
        config()
    }
}

use i18n::{get_message, get_message_with_args};

/// Helper function for templates to translate strings
pub fn translate(message_id: &str) -> String {
    get_message(message_id)
}

/// Helper function for templates to translate strings with arguments
pub fn translate_with_args(message_id: &str, name: &str, count: &str) -> String {
    get_message_with_args(message_id, name, count)
}

/// Macro for easy translation in Rust code
#[macro_export]
macro_rules! t {
    ($message_id:expr) => {
        crate::i18n_helper::translate($message_id)
    };
    ($message_id:expr, $name:expr, $count:expr) => {
        crate::i18n_helper::translate_with_args($message_id, $name, $count)
    };
}

use warpui_core::AppContext;

pub fn text_for_app_or(_app: &AppContext, _key: &str, fallback: &str) -> String {
    fallback.to_owned()
}

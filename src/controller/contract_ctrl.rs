use axum::response::Html;
use crate::view::render_contract_page;

pub async fn contract_page() -> Html<String> {
    Html(render_contract_page())
}
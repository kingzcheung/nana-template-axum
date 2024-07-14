use nana_template_axum::{config::Config, server::start};
use tracing::Level;
use tracing_subscriber::fmt::format;

#[tokio::main]
async fn main() {
    let conf = envy::from_env::<Config>().expect("环境配置解析错误");
    tracing_subscriber::fmt().event_format(format().compact()).with_max_level(Level::INFO).init();
    tokio::join!(start(conf));
}
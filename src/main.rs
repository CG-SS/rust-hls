use tide::{log, Request};

#[derive(Clone)]
struct AppConfig {
    admin_username: String,
    admin_password: String,
    hostname: String,
    port: u16,
}

#[derive(Clone)]
struct AppState {
    config: AppConfig
}

fn get_config_from_env() -> AppConfig {
    let admin_username = 
        std::env::var("ADMIN_USER")
        .unwrap_or("admin".to_string());
    let admin_password = 
        std::env::var("ADMIN_PASSWORD")
        .unwrap_or("admin".to_string());
    let hostname = 
        std::env::var("HOST")
        .unwrap_or("0.0.0.0".to_string());
    let port: u16 = 
        std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("PORT to be a number");
    
    AppConfig { 
        admin_username, 
        admin_password, 
        hostname, 
        port
    }
}

async fn start_app(app_config: AppConfig) -> tide::Result<()> {
    let hostname = app_config.hostname.clone();
    let port = app_config.port.clone();

    let mut app = tide::with_state(AppState {
        config: app_config,
    });
    app.at("/health").get(get_health);

    app.listen(format!("{}:{}", hostname, port)).await?;

    Ok(())
}

async fn get_health(_req: Request<AppState>) -> tide::Result {
    Ok(format!("Up and running!").into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();
    
    let app_config = get_config_from_env();
    let r = start_app(app_config).await;

    r
}
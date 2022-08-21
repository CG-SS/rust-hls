use tide::Request;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/health").get(get_health);

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

async fn get_health(mut _req: Request<()>) -> tide::Result {
    Ok(format!("Up and running!").into())
}
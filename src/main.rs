use messages_actix::MessageApp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let port = 8080;
    let app = MessageApp::new(port);
    app.run().await
}

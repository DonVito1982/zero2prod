use std::net::TcpListener;
use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let fail_text = "Failed to bind random port";
    let listener = TcpListener::bind("127.0.0.1:8000").expect(fail_text);
    run(listener)?.await
}

use std::time::Duration;

use tokio::sync::{broadcast, oneshot};
use tokio::time::sleep;

use async_uws::app::App;
use async_uws::http_request::HttpRequest;
use async_uws::http_response::HttpResponse;
use async_uws::uwebsockets_rs::UsSocketContextOptions;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let opts = UsSocketContextOptions {
        key_file_name: None,
        cert_file_name: None,
        passphrase: None,
        dh_params_file_name: None,
        ca_file_name: None,
        ssl_ciphers: None,
        ssl_prefer_low_memory_usage: None,
    };

    let mut app = App::new(opts, None);

    app.get("/", get_handler)
        .listen(
            3001,
            Some(|listen_socket| {
                println!("{listen_socket:#?}");
            }),
        )
        .run();
    println!("Server exiting");
}

async fn get_handler(res: HttpResponse<false>, req: HttpRequest) {
    res.end(Some("Hello World".into()), true);
}

//! Demo of the WINATEP client.
use winatep_client::Session;

#[tokio::main]
async fn main() {
    env_logger::builder().init();

    log::info!("starting the demo");

    let mut session = match Session::new("http://127.0.0.1:31337/driver").await {
        Ok(s) => s,
        Err(err) => {
            log::error!("{err}");
            log::info!("...maybe the driver isn't running?");
            panic!("cannot continue without a session");
        }
    };

    let screens = session.get_screens().await.unwrap();
    log::info!("got screens: {screens:#?}");
}

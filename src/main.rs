use std::{error::Error, time::Duration};
use tokio::{sync::mpsc, task::spawn_blocking, time};

#[hot_lib_reloader::hot_module(dylib = "lib")]
mod hot_lib {
    hot_functions_from_file!("lib/src/lib.rs");

    pub use lib::State;

    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut state = hot_lib::init_game();

    let (tx, mut rx) = mpsc::channel(1);

    tokio::spawn(async move {
        loop {
            let block_reload = spawn_blocking(|| hot_lib::subscribe().wait_for_about_to_reload())
                .await
                .expect("Get token");
            tx.send(block_reload).await.expect("Send token");
        }
    });

    loop {
        tokio::select! {
            _ = time::sleep(Duration::from_secs(1)) => {
                hot_lib::draw_game(&state);
                hot_lib::update_game(&mut state);

            }
            Some(block_reload_token) = rx.recv() => {
                drop(block_reload_token);
                hot_lib::subscribe().wait_for_reload();
                state = hot_lib::init_game();

        }
            }
    }
}

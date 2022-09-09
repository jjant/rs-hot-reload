#[hot_lib_reloader::hot_module(dylib = "lib")]
mod hot_lib {
    hot_functions_from_file!("lib/src/lib.rs");

    pub use framework::Game;
    pub use lib::State;
}

fn main() {
    let mut state = <hot_lib::State as hot_lib::Game>::init();

    loop {
        hot_lib::update_game(&mut state);
        hot_lib::draw_game(&state);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

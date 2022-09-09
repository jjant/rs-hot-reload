pub trait Game {
    fn init() -> Self;
    fn update(&mut self);
    fn draw(&self);
}

#[macro_export]
macro_rules! make_exports {
    ($t: ty) => {
        #[no_mangle]
        pub fn init_game() -> $t {
            <$t as Game>::init()
        }

        #[no_mangle]
        pub fn draw_game(t: &$t) {
            <$t as Game>::draw(t);
        }
    };
}

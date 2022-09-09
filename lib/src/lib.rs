use framework::Game;

#[derive(Debug)]
pub struct State {
    value: i64,
}

impl Game for State {
    fn init() -> Self {
        Self { value: 42 }
    }

    fn update(&mut self) {
        self.value += 8;
    }

    fn draw(&self) {
        println!("{:?}", self);
    }
}

#[no_mangle]
pub fn init_game() {
    println!("wtf");
    <State as Game>::init();
}

#[no_mangle]
pub fn update_game(t: &mut State) {
    println!("Updating game");
    <State as Game>::update(t);
}

#[no_mangle]
pub fn draw_game(t: &State) {
    println!("Drawing game");
    <State as Game>::draw(t);
}

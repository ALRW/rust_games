use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(1, "Hello World!")
    }

}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Rusty Roguelike")
        .build()?;
    main_loop(context, State{})
}

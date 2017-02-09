extern crate amethyst;

use amethyst::engine::{Application, State, Trans};
use amethyst::specs::World;
use amethyst::gfx_device::DisplayConfig;
use amethyst::asset_manager::AssetManager;
use amethyst::renderer::Pipeline;

struct HelloWorld;

impl State for HelloWorld {
    fn on_start(&mut self, _: &mut World, _: &mut AssetManager, _: &mut Pipeline) {
        println!("Game started!");
    }

    fn update(&mut self, _: &mut World, _: &mut AssetManager, _: &mut Pipeline) -> Trans {
        println!("Hello from Amethyst!");
        Trans::Quit
    }

    fn on_stop(&mut self, _: &mut World, _: &mut AssetManager, _: &mut Pipeline) {
        println!("Game stopped!");
    }
}

fn main() {
    let display_config = DisplayConfig::default();
    let mut game = Application::build(HelloWorld, display_config).done();
    game.run();
}

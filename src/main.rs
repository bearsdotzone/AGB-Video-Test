// Games made using `agb` are no_std which means you don't have access to the standard
// rust library. This is because the game boy advance doesn't really have an operating
// system, so most of the content of the standard library doesn't apply.
//
// Provided you haven't disabled it, agb does provide an allocator, so it is possible
// to use both the `core` and the `alloc` built in crates.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Failing to do so will cause failure in linking
// which won't be a particularly clear error message.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{
    display::{
        tiled::{RegularBackgroundSize, TileFormat, TileSet, TileSetting, TiledMap},
        Priority,
    },
    include_gfx, syscall,
};

agb::include_gfx!("gfx/beargba.toml");

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let gfx = gba.display.object.get();
    let vblank = agb::interrupt::VBlank::get();
    let (tiled, mut vram) = gba.display.video.tiled0();
    let tileset = TileSet::new(beargba::title.tiles, TileFormat::FourBpp);

    vram.set_background_palettes(beargba::PALETTES);

    let mut bg = tiled.background(Priority::P0, RegularBackgroundSize::Background32x32);

    for y in 0..20u16 {
        for x in 0..30u16 {
            bg.set_tile(
                &mut vram,
                (x, y).into(),
                &tileset,
                TileSetting::new(x + y * 30u16, false, false, 0),
            );
        }
    }
    bg.commit(&mut vram);
    bg.show();
    loop {
        syscall::halt();
    }
}

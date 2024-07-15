
use agb::display::tiled::{RegularMap, tiledMap, VRamManager};

agb::include_background_gfx!(splash_screens,
                             splash => deduplicate "gfx/title.png",
                             );

pub enum SplashScreen {
    Title,
    End,
}

pub fn show_splash_screen(
    which: SplashScreen,
    //sfx: &mut SFxPlayer,
    map: &mut Regularmap,
    vram: &mut VRamManager,

    ) {

    map.set_scroll_pos((0i16, 0i16));

    let tile_data = match which {
        SplashScreen::Title => &splash_screens::title,
    };

    let vblank = agb::interrupt::VBlank::get();
    let mut input = agbz::input::ButtonController::new();

    vblank.wait_for_vblank();

    map.fill_with(vram, tiled_data);

    map.commit(vram);


}



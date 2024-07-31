#![no_std]
#![no_main]

extern crate alloc;
mod title;

pub fn main(mut agb: agb::Gba) -> ! {

    let (tiled, mut vram) = agb.display.video.tiled0();
    
    //what is tilesheet
    vram.set_background_palettes(tile_sheet::PALETTES);

    //create 2 backgrounds
    //fill world_display (bg2) with tiles via set_tile
    //
    //mixer/sfx stuff
    //
    //commit world_display to vram
    //show splash
    //
    //game loop
    //

}



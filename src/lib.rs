#![no_std]
#![no_main]

extern crate alloc;


pub fn mian(mut agb: agb::Gba) -> ! {
    let (tiled, mut vram) = agb.display.video.tiled0();
    vram.set;
}

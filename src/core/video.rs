pub fn draw(gfx: &[u8]) {
    print!("{esc}[1;1H", esc = 27 as char);

    for y in 0..32 {
        for x in 0..64 {
            print!(
                "{}",
                if gfx[(y * 64 + x) as usize] == 1 {
                    "█"
                } else {
                    " "
                }
            );
        }
        println!();
    }
}

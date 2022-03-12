use rendering::{ RenderBuffer, RenderOutput, WledRenderOutput };

fn main() {
    let mut log_lvl = simplelog::LevelFilter::Warn;
    #[cfg(debug_assertions)]
    { log_lvl = simplelog::LevelFilter::Debug; }
    simplelog::TermLogger::init(
        log_lvl,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Always
    ).unwrap();

    let mut buffer = RenderBuffer::new(36);
    let mut output = WledRenderOutput::new(36, "192.168.1.40", 21324).expect("Could not create output");
    let red = color::RgbF32{red: 1.0, green: 0.0,  blue: 0.0 };
    let black = color::RgbF32::black();
    buffer.draw_range(2, &[red, black, black, black, red]);
    let res = output.draw(&buffer);
    if let Err(err) = res {
        log::error!("Failed to draw to output: {}", err);
    }
}

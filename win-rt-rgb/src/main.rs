use rendering::{ RenderBuffer, RenderOutput, WledRenderOutput };
use futures::StreamExt;
use simple_error::SimpleError;
use log::info;
use frame_sampler::FrameSampler;

mod transformations;
mod device;
mod frame_sampler;

mod config {
    pub const DESKTOP_CAPTURE_FPS: f32 = 15.0;
}


fn main() {
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Always
    ).unwrap();
    info!("Starting application");

    let capturer = desktop_capture::DesktopCaptureController::new(config::DESKTOP_CAPTURE_FPS);
    let mut device = create_wled_device(&capturer, "192.168.1.6", 21324, 9).unwrap();

    let task = async {
        while let Some(frame) = device.stream.next().await {
            device.output.draw(&frame).unwrap();
        }
    };
    futures::executor::block_on(task);
}

struct Device<'a> {
    pub stream: transformations::BufferStream<'a>,
    pub output: Box<dyn RenderOutput>,
}

fn create_wled_device<'a>(capturer: &desktop_capture::DesktopCaptureController, address: &'static str, port: u32, n_leds: usize) -> Result<Device<'a>, SimpleError> {
    let output = WledRenderOutput::new(n_leds, address, port)?;
    let mut sampler = frame_sampler::HorizontalFrameSampler{buffer: RenderBuffer::new(n_leds)};
    let mut stream = capturer.subscribe().map(move |f| sampler.sample(&f.unwrap())).boxed();
    stream = transformations::map(stream, |mut buf| { buf.data[0].red = 1.0; buf });
    Ok(Device {
        stream: stream,
        output: Box::new(output),
    })
}

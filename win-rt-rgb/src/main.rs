use rendering::{ RenderBuffer, RenderOutput, WledRenderOutput };

mod pipeline;

fn main() {
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Always
    ).unwrap();

    // let mut buffer = RenderBuffer::new(36);
    let mut output = WledRenderOutput::new(9, "192.168.1.6", 21324).expect("Could not create output");
    let mut capturer = desktop_capture::DesktopCaptureController::new().expect("Could not create capture controller");

    let mut pipeline = pipeline::Pipeline{
        steps: Vec::new(),
        sampler: Box::new(pipeline::frame_sampler::HorizontalFrameSampler{
            buffer: RenderBuffer::new(9),
        })
    };
    loop {
        // std::thread::sleep(std::time::Duration::from_millis(200));
        let frame = capturer.get_frame();
        if let Err(err) = frame {
            log::error!("Failed to capture frame: {}", err);
            continue;
        }
        let frame_data = frame.unwrap();
        let actual_frame = pipeline::frame_sampler::Frame{
            buffer: frame_data.0,
            width: frame_data.1.0,
            height: frame_data.1.1,
        };
        let buf = pipeline.build(&actual_frame);
        let res = output.draw(buf);
        if let Err(err) = res {
            log::error!("Failed to draw to output: {}", err);
        }
    }
}

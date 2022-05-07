
use futures::TryStreamExt;
use log::{info, debug, warn};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc};
use tokio_stream::Stream;
use tokio_stream::wrappers::ReceiverStream;
use std::future::Future;
use std::net::SocketAddr;
use simple_error::{SimpleError, SimpleResult, try_with};

use crate::render_service::{RenderOutput, specification::{DeviceSpecification, SamplingType, AudioSamplingParameters, HsvAdjustment}};
use crate::outputs::{WledRenderOutput, QmkRenderOutput};
use crate::profiles::{self, ApplicationProfile};

pub enum Frame {
    Devices(Vec<DeviceSpecification>),
    Profiles(Vec<profiles::ApplicationProfile>),
}

mod deser_types {
    #[derive(serde::Deserialize)]
    pub struct Message {
        pub subject: String,
    }

    #[derive(serde::Deserialize)]
    pub struct DeviceMessage {
        pub subject: String,
        pub contents: Vec<DeviceEntry>,
    }
    #[derive(serde::Deserialize)]
    pub struct DeviceEntry {
        pub enabled: bool,
        pub device: DeviceSpec,
    }
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DeviceSpec {
        pub name: String,
        pub number_of_leds: u32,
        pub gamma: f32,
        pub color_temp: u32,
        pub saturation_adjustment: u32,
        pub value_adjustment: u32,
        pub audio_amount: f32,
        #[serde(rename = "type")]
        pub variant: u32,
        pub wled_data: Option<WledData>,
        pub qmk_data: Option<QmkData>,
    }
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WledData {
        pub ip_address: String,
    }
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct QmkData {
        pub vendor_id: u16,
        pub product_id: u16,
    }

    #[derive(serde::Deserialize)]
    pub struct ProfileMessage {
        pub subject: String,
        pub contents: Vec<ProfileEntry>,
    }
    #[derive(serde::Deserialize)]
    pub struct ProfileEntry {
        pub id: u32,
        pub regex: String,
        pub areas: Vec<AreaSpecification>,
        pub priority: i32,
    }
    #[derive(serde::Deserialize)]
    pub struct AreaSpecification {
        pub selector: Option<MonitorDimensions>,
        pub width: MonitorDistance,
        pub height: MonitorDistance,
        pub x: MonitorDistance,
        pub y: MonitorDistance,
    }
    #[derive(serde::Deserialize)]
    pub struct MonitorDimensions {
        pub width: usize,
        pub height: usize,
    }
    #[derive(serde::Deserialize)]
    pub struct MonitorDistance {
        pub px: Option<i32>,
        pub percentage: Option<f32>,
    }
}

pub async fn run_websocket_server(port: u32, mut shutdown: broadcast::Receiver<()>) -> SimpleResult<(impl Future<Output=()>, impl Stream<Item=Frame>)> {
    let (frame_tx, frame_rx) = mpsc::channel(16);
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).await.map_err(SimpleError::from)?;
    info!("Websocket listening on: {}", &addr);
    let task = async move {
        loop {
            tokio::select! {
                conn = listener.accept() => {
                    if let Ok((stream, client_addr)) = conn {
                        tokio::spawn(handle_connection(stream, frame_tx.clone(), client_addr));
                    } else {
                        break;
                    }
                },
                _ = shutdown.recv() => {
                    break;
                }
            };
        }
        debug!("Shutting down websocket server");
    };
    Ok((task, ReceiverStream::new(frame_rx)))
}

async fn handle_connection(raw_stream: TcpStream, frame_tx: mpsc::Sender<Frame>, client_addr: SocketAddr) {
    info!("Accepted connection from: {}", client_addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");

    ws_stream.try_for_each(|raw_msg| async {
        let str_msg = raw_msg.into_text().unwrap();
        if let Ok(msg) = serde_json::from_str::<deser_types::Message>(&str_msg) {
            debug!("Received '{}' message from {}", msg.subject, client_addr);
            let frame = match msg.subject.as_str() {
                "devices" => {
                    match serde_json::from_str::<deser_types::DeviceMessage>(&str_msg) {
                        Ok(device_msg) => Some(handle_device_message(device_msg)),
                        Err(e) => {
                            warn!("Failed to parse message: {}", e);
                            None
                        },
                    }
                },
                "profiles" => {
                    match serde_json::from_str::<deser_types::ProfileMessage>(&str_msg) {
                        Ok(profile_msg) => Some(handle_profile_message(profile_msg)),
                        Err(e) => {
                            warn!("Failed to parse message: {}", e);
                            None
                        },
                    }
                },
                _ => {
                    warn!("Received unknown message subject '{}'", &msg.subject);
                    None
                }
            };
            if let Some(f) = frame {
                match frame_tx.send(f).await {
                    Ok(()) => (),
                    Err(_) => {
                        warn!("Received websocket message after, but handler has already been closed");
                        return Err(tokio_tungstenite::tungstenite::Error::ConnectionClosed);
                    },
                }
            }
        };

        Ok(())
    }).await.unwrap();
    info!("Disconnected from {}", client_addr);
}

fn handle_device_message(msg: deser_types::DeviceMessage) -> Frame {
    let enabled = msg.contents.into_iter().filter(|dev| dev.enabled);
    let mut device_specs = Vec::new();
    for (i, entry) in enabled.into_iter().enumerate() {
        let output: SimpleResult<Box<dyn RenderOutput + Send>> = match &entry.device.variant {
            0 => {
                match entry.device.wled_data {
                    Some(wled_params) => WledRenderOutput::new(
                        entry.device.number_of_leds as usize,
                        wled_params.ip_address.clone(),
                        21324
                    ).map(|out| -> Box<dyn RenderOutput + Send> { Box::new(out) }),
                    None => Err(SimpleError::new("Expected WLED parameters, but none were supplied")),
                }
            },
            1 => {
                match entry.device.qmk_data {
                    Some(qmk_params) => QmkRenderOutput::new(
                        entry.device.number_of_leds as usize,
                        qmk_params.vendor_id,
                        qmk_params.product_id
                    ).map(|out| -> Box<dyn RenderOutput + Send> { Box::new(out) }),
                    None => Err(SimpleError::new("Expected WLED parameters, but none were supplied")),
                }
            },
            v => Err(SimpleError::new(format!("Unsupported device variant {}", v))),
        };
        match output {
            Ok(out) => device_specs.push(DeviceSpecification {
                    output: out,
                    sampling_type: SamplingType::Horizontal,
                    hsv_adjustments: Some(HsvAdjustment{ hue: 0.0, value: entry.device.value_adjustment as f32 / 100.0, saturation: entry.device.saturation_adjustment as f32 / 100.0}),
                    smoothing: None,
                    audio_sampling: if entry.device.audio_amount > 0.0 { Some(AudioSamplingParameters{ amount: entry.device.audio_amount / 100.0 }) } else { None },
                    gamma: entry.device.gamma,
                }),
            Err(e) => {
                warn!("Skipping device with index {}: {}", i, e)
            }
        };
    }

    Frame::Devices(device_specs)
}

fn handle_profile_message(msg: deser_types::ProfileMessage) -> Frame {
    let mut new_profiles = Vec::new();
    for profile_raw in msg.contents {
        let profile = parse_profile(&profile_raw);
        match profile {
            Ok(profile) => new_profiles.push(profile),
            Err(e) => warn!("Skipping profile '{}': {}", profile_raw.regex, e),
        }

    };
    Frame::Profiles(new_profiles)
}

fn parse_profile(profile_raw: &deser_types::ProfileEntry) -> SimpleResult<ApplicationProfile> {
    let regex = try_with!(regex::Regex::new(&profile_raw.regex), "Invalid title regex");
    let mut areas = Vec::new();
    for area_raw in &profile_raw.areas {
        let resolution = area_raw.selector.as_ref().map(|dim| (dim.width, dim.height));
        areas.push(profiles::MonitorAreaSpecification{
            resolution,
            left: parse_monitor_distance(&area_raw.x)?,
            top: parse_monitor_distance(&area_raw.y)?,
            width: parse_monitor_distance(&area_raw.width)?,
            height: parse_monitor_distance(&area_raw.height)?,
        })
    }
    Ok(profiles::ApplicationProfile{
        id: profile_raw.id,
        priority: profile_raw.priority,
        title_regex: regex,
        areas,
    })
}

fn parse_monitor_distance(distance_raw: &deser_types::MonitorDistance) -> SimpleResult<profiles::MonitorDistance> {
    if distance_raw.px.is_none() && distance_raw.percentage.is_none() {
        return Err(SimpleError::new("Area must specify either px or percentage"));
    }
    if let Some(px) = distance_raw.px {
        Ok(profiles::MonitorDistance::Pixels(px as isize))
    } else {
        let percentage = distance_raw.percentage.unwrap() / 100.0;
        if !(0.0..=1.0).contains(&percentage) {
            return Err(SimpleError::new(format!("Area percentage must be in [0.0, 1.0], was {}", percentage)));
        }
        Ok(profiles::MonitorDistance::Proportion(percentage))
    }
}
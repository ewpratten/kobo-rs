use std::fs::File;

use chrono::{DateTime, Duration, Utc};
use evdev_rs::{Device, InputEvent, ReadFlag, ReadStatus};

use crate::coords::PixelSpaceCoord;

/// Describes a touch event.
#[derive(Debug, Clone)]
pub struct Touch {
    /// The touch position
    pub position: PixelSpaceCoord,
    /// The touch pressure
    pub pressure: i32,
    /// The timestamp of the touch event
    pub timestamp: DateTime<Utc>,
}

impl Touch {
    /// Checks if the touch is a finger-down event
    pub fn is_down(&self) -> bool {
        self.pressure > 0
    }
}

/// Blocking event listener for touch events
pub struct TouchEventListener {
    device: Device,
}

impl TouchEventListener {
    /// Construct a new `TouchEventListener` by opening the event stream
    pub fn open() -> std::io::Result<Self> {
        // Open the touch device
        let file = File::open("/dev/input/event1")?;
        let device = Device::new_from_file(file)?;

        Ok(Self { device })
    }

    /// Read the next event from the stream
    pub fn next_raw_event(&self) -> std::io::Result<(ReadStatus, InputEvent)> {
        self.device
            .next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING)
    }

    /// Read the next touch event
    ///
    /// ## Notes
    ///
    /// While this will attempt to stop at a timeout, there is a chance the event stream will just block us past it anyways.
    ///
    /// Pressure is currently unreliable, so we'll just assume it's always down.
    pub fn next_touch(
        &self,
        screen_size: PixelSpaceCoord,
        timeout: Option<Duration>,
    ) -> Option<Touch> {
        // Keep track of the start time
        let start = Utc::now();

        // Holder for out data
        let mut x = None;
        let mut y = None;
        let mut pressure = None;

        // Loop through the incoming event stream
        loop {
            // Check the timeout
            if let Some(timeout) = timeout {
                let elapsed = Utc::now().signed_duration_since(start);
                if elapsed > timeout {
                    return None;
                }
            }

            // Read the next event
            let (status, event) = self.next_raw_event().unwrap();


            // Check the status
            if status == ReadStatus::Success {
                // We are looking for ABS touch events
                match event.event_code {
                    evdev_rs::enums::EventCode::EV_ABS(kind) => match kind {
                        // Note:
                        // X and Y are swapped from the event stream
                        // X is also inverted
                        evdev_rs::enums::EV_ABS::ABS_X => {
                            y = Some(event.value);
                        }
                        evdev_rs::enums::EV_ABS::ABS_Y => {
                            x = Some(screen_size.x - event.value);
                        }
                        evdev_rs::enums::EV_ABS::ABS_MT_POSITION_X => {
                            y = Some(event.value);
                        }
                        evdev_rs::enums::EV_ABS::ABS_MT_POSITION_Y => {
                            x = Some(screen_size.x - event.value);
                        }
                        evdev_rs::enums::EV_ABS::ABS_PRESSURE => {
                            pressure = Some(event.value);
                        }
                        _ => {}
                    },
                    _ => { /* Unused */ }
                }
            }

            // Check if we have all the data
            if x.is_some() && y.is_some() && pressure.is_some() {
                // Return the touch event
                return Some(Touch {
                    position: PixelSpaceCoord::new(x.unwrap(), y.unwrap()),
                    pressure: pressure.unwrap(),
                    timestamp: Utc::now(),
                });
            }
        }
    }
}

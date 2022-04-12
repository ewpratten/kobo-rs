use std::{
    path::PathBuf,
    process::{Command, ExitStatus},
};

use regex::Regex;

use crate::coords::{CharSpaceCoord, PixelSpaceCoord};

use super::{color::EinkColor, error::DisplayError};

const FBINK_VAR_PARSE: &str =
    r"screenWidth=(\d+).*screenHeight=(\d+).*MAXCOLS=(\d+).*MAXROWS=(\d+)";

/// A simple character-only display controller
///
/// # Debugging FBInk
///
/// You can set `KOBO_RS_DEBUG_FBINK=1` in your environment to enable debug printing for the FBInk subprocess
pub struct CharacterDisplay {
    /// Executable path of FBInk
    fbink_path: PathBuf,
    /// The size of the display in pixels
    display_size: PixelSpaceCoord,
    /// The size of the display in characters
    display_char_size: CharSpaceCoord,
}

impl CharacterDisplay {
    /// Opens a new character display through FBInk
    pub fn open() -> Result<Self, DisplayError> {
        // Check for FBInk
        let fbink_install_locations: Vec<PathBuf> = vec![
            "/usr/local/kfmon/bin/fbink".parse().unwrap(),
            "/usr/bin/fbink".parse().unwrap(),
        ];
        let fbink_path = fbink_install_locations
            .iter()
            .find(|path| path.exists())
            .ok_or(DisplayError::FBInkNotFound)?;

        // Call FBInk to get the internal variables
        let output = Command::new(fbink_path).arg("-e").output().unwrap();
        let o = String::from_utf8(output.stdout).unwrap();
        let matched = Regex::new(FBINK_VAR_PARSE)
            .unwrap()
            .captures(o.trim())
            .unwrap();

        Ok(Self {
            fbink_path: fbink_path.to_owned(),
            display_size: PixelSpaceCoord::new(
                matched[1].parse().unwrap(),
                matched[2].parse().unwrap(),
            ),
            display_char_size: CharSpaceCoord::new(
                matched[3].parse().unwrap(),
                matched[4].parse().unwrap(),
            ),
        })
    }

    /// Execute a command through FBInk
    fn fbink_exec(&self, args: Vec<String>) -> std::io::Result<ExitStatus> {
        let mut cmd = Command::new(&self.fbink_path);
        cmd.args(args);

        // If the `KOBO_RS_DEBUG_FBINK` environment variable is set, pass the command output to stdout otherwise keep it silent
        if std::env::var("KOBO_RS_DEBUG_FBINK").is_ok() {
            cmd.stdout(std::process::Stdio::inherit());
        } else {
            cmd.stdout(std::process::Stdio::null());
        }

        Ok(cmd.output()?.status)
    }

    /// Get the screen size in pixels
    pub fn get_screen_size_px(&self) -> PixelSpaceCoord {
        self.display_size
    }

    /// Get the screen size in characters
    pub fn get_screen_size_ch(&self) -> CharSpaceCoord {
        self.display_char_size
    }

    /// Clear the display
    pub fn clear_screen(&self) -> std::io::Result<ExitStatus> {
        self.fbink_exec(vec!["-c".to_string()])
    }

    /// Write a string to a specific display position
    pub fn write_str(
        &self,
        s: &str,
        pos: CharSpaceCoord,
        invert_colors: bool,
        color: Option<EinkColor>,
        background_color: Option<EinkColor>,
    ) -> std::io::Result<ExitStatus> {
        let mut args = vec![
            "-y".to_string(),
            pos.y.to_string(),
            "-x".to_string(),
            pos.x.to_string(),
            "-C".to_string(),
            color.unwrap_or(EinkColor::Black).to_string(),
            "-B".to_string(),
            background_color.unwrap_or(EinkColor::White).to_string(),
            s.to_string(),
        ];

        if invert_colors {
            args.push("--invert".to_string());
        }

        self.fbink_exec(args)
    }
}

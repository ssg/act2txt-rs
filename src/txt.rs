use std::io::Write;

use crate::act::Palette;

pub enum WriteError {
    IoError,
}

impl From<std::io::Error> for WriteError {
    fn from(_: std::io::Error) -> Self {
        WriteError::IoError
    }
}

impl Palette {
    /// Write the palette to a Paint.NET TXT file.
    pub fn write_pdn_txt<T: Write>(&self, output: &mut T) -> Result<(), WriteError> {        
        writeln!(output, "; Created by act2txt v{} - https://github.com/ssg/act2txt-rs", env!("CARGO_PKG_VERSION"))?;
        if let Some(ed) = &self.extra_data {
            writeln!(output, "; ACT number of colors = {}", ed.num_colors)?;
            writeln!(output, "; ACT transparent color index = {}", ed.transparent_index)?;
        }
        for color in &self.colors {
            writeln!(output, "{:02X}{:02X}{:02X}", color.red, color.green, color.blue)?;
        }
        Ok(())
    }
}
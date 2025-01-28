use std::io::Write;

use crate::act::Palette;

#[derive(Debug)]
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
        writeln!(output, "; Number of colors = {}", self.colors.len())?;
        if let Some(index) = &self.transparent_index {
            writeln!(output, "; ACT transparent color index = {}", index)?;
        }
        for color in &self.colors {
            writeln!(output, "{:02X}{:02X}{:02X}", color.red, color.green, color.blue)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    
    use crate::act::Palette;
    use palette::rgb::Srgb;
    
    #[test]
    fn write_pdn_txt_with_transparent_index() {
        let palette = Palette {
            colors: vec![
                Srgb::new(0, 0, 0),
                Srgb::new(255, 255, 255),
            ],
            transparent_index: Some(1),
        };

        let mut output = Vec::new();
        palette.write_pdn_txt(&mut output).unwrap();
        let output_str = String::from_utf8(output).unwrap();

        let expected_output = format!(
            "; Created by act2txt v{} - https://github.com/ssg/act2txt-rs\n\
             ; Number of colors = 2\n\
             ; ACT transparent color index = 1\n\
             000000\n\
             FFFFFF\n",
            env!("CARGO_PKG_VERSION")
        );

        assert_eq!(output_str, expected_output);
    }

    #[test]
    fn test_write_pdn_txt_no_transparent_index() {
        let palette = Palette {
            colors: vec![
                Srgb::new(0, 0, 0),
                Srgb::new(255, 0, 0),
            ],
            transparent_index: None,
        };

        let mut output = Vec::new();
        palette.write_pdn_txt(&mut output).unwrap();
        let output_str = String::from_utf8(output).unwrap();

        let expected_output = format!(
            "; Created by act2txt v{} - https://github.com/ssg/act2txt-rs\n\
             ; Number of colors = 2\n\
             000000\n\
             FF0000\n",
            env!("CARGO_PKG_VERSION")
        );

        assert_eq!(output_str, expected_output);
    }

    #[test]
    fn test_write_pdn_txt_empty_palette() {
        let palette = Palette {
            colors: vec![],
            transparent_index: None,
        };

        let mut output = Vec::new();
        palette.write_pdn_txt(&mut output).unwrap();
        let output_str = String::from_utf8(output).unwrap();

        let expected_output = format!(
            "; Created by act2txt v{} - https://github.com/ssg/act2txt-rs\n\
             ; Number of colors = 0\n",
            env!("CARGO_PKG_VERSION")
        );

        assert_eq!(output_str, expected_output);
    }
}
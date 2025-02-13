use core::fmt::Formatter;
use std::io::Read;

use palette::Srgb;

pub const MAX_COLOR_BUFFER_LENGTH: usize = 768;
pub const MAX_COLORS: usize = 256;
pub const EXTRA_DATA_SIZE: usize = 4;

#[derive(Debug)]
pub struct Palette {
    pub colors: Vec<Srgb<u8>>,
    pub transparent_index: Option<u16>,
}

#[derive(Debug)]
pub enum ReadError {
    InvalidFileLength,
    IoError(std::io::Error),
}

impl core::fmt::Display for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ReadError::InvalidFileLength => "Invalid file length",
            ReadError::IoError(_) => "I/O error",
        })
    }
}

impl core::error::Error for ReadError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
    
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ReadError::InvalidFileLength => None,
            ReadError::IoError(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for ReadError {
    fn from(err: std::io::Error) -> Self {
        ReadError::IoError(err)
    }
}

impl Palette {
    pub fn read(reader: &mut impl Read, all: bool) -> Result<Palette, ReadError> {
        let mut buf = [0; MAX_COLOR_BUFFER_LENGTH];
        reader
            .read_exact(&mut buf)
            .map_err(|_| ReadError::InvalidFileLength)?;

        let (transparent_index, num_colors) = read_extra_data(reader, all);
        let colors = buf
            .chunks_exact(3)
            .take(num_colors)
            .map(|chunk| Srgb::new(chunk[0], chunk[1], chunk[2]))
            .collect();

        Ok(Palette { colors, transparent_index })
    }
}

fn read_extra_data(reader: &mut (impl Read + ?Sized), all: bool) -> (Option<u16>, usize) {
    let mut extra_buf = [0; EXTRA_DATA_SIZE];
    let mut transparent_index = None;
    let mut num_colors = MAX_COLORS;
    if reader.read_exact(&mut extra_buf).is_ok() {
        if !all {
            num_colors = u16::from_le_bytes([extra_buf[0], extra_buf[1]]) as usize;
        }
        transparent_index = Some(u16::from_le_bytes([extra_buf[2], extra_buf[3]]));
    }
    (transparent_index, num_colors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn read_with_extra_data_returns_data_and_extra_data() {
        let mut data = generate_colors();
        data.extend_from_slice(&[2, 0, 1, 0]); // extra data: num_colors = 2, transparent_index = 1
        let mut reader = Cursor::new(data);
        let palette = Palette::read(&mut reader, false).unwrap();
        assert_eq!(palette.colors.len(), 2);
        assert_color_values(&palette);
        assert!(matches!(palette.transparent_index, Some(1)));
    }

    fn assert_color_values(palette: &Palette) {
        palette.colors.iter().enumerate().for_each(|(i, color)| {
            assert_eq!(*color, Srgb::new(i as u8, (i * 2) as u8, (i * 3) as u8));
        });
    }
    
    #[test]
    fn read_with_extra_data_forced_all_returns_data_with_all_colors() {
        let mut data = generate_colors();
        data.extend_from_slice(&[2, 0, 0, 1]); // extra data: num_colors = 2, transparent_index = 1
        let mut reader = Cursor::new(data);
        let palette = Palette::read(&mut reader, true).unwrap();
        assert_eq!(palette.colors.len(), 256);
        assert_color_values(&palette);
        assert!(matches!(palette.transparent_index, Some(256)));
    }

    #[test]
    fn read_no_extra_data_returns_only_data() {
        let data = generate_colors();
        let mut reader = Cursor::new(data);
        let palette = Palette::read(&mut reader, false).unwrap();
        assert_eq!(palette.colors.len(), 256);
        for i in 0..256 {
            assert_eq!(palette.colors[i], Srgb::new(i as u8, (i * 2) as u8, (i * 3) as u8));
        }
        assert!(palette.transparent_index.is_none());
    }

    #[test]
    fn test_palette_read_invalid_length() {
        for i in 0..MAX_COLOR_BUFFER_LENGTH {
            let data = vec![0; i];
            let mut reader = Cursor::new(data);
            let result = Palette::read(&mut reader, false);
            assert!(matches!(result, Err(ReadError::InvalidFileLength)));
        }
    }

    fn generate_colors() -> Vec<u8> {
        let mut data = vec![0; 768];
        // 256 colors
        for i in 0..256 {
            data[i * 3] = i as u8;
            data[i * 3 + 1] = (i * 2) as u8;
            data[i * 3 + 2] = (i * 3) as u8;
        }
        data
    }
}

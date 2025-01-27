use std::{fs::File, io::Read};

use byteorder::{LittleEndian, ReadBytesExt};
use palette::Srgb;

pub const MAX_FILE_LENGTH: usize = 768;
pub const MAX_COLOR_BUFFER_LENGTH: usize = MAX_FILE_LENGTH;
pub const MAX_FILE_LENGTH_WITH_EXTRA_DATA: usize = 772;
pub const MAX_COLORS: usize = 256;

#[derive(Debug)]
pub struct Palette {
    pub colors: Vec<Srgb<u8>>,
    pub extra_data: Option<ExtraData>,
}

#[derive(Default, Debug)]
pub struct ExtraData {
    pub num_colors: u16,
    pub transparent_index: u16,
}

#[derive(Debug)]
pub enum ReadError {
    InvalidFileLength,
    IoError,
}

impl From<std::io::Error> for ReadError {
    fn from(_: std::io::Error) -> Self {
        ReadError::IoError
    }
}

impl Palette {
    pub fn read(file: &mut File, all: bool) -> Result<Palette, ReadError> {
        let has_extra = has_extra_data(file)?;

        let mut buf = [0; MAX_COLOR_BUFFER_LENGTH];
        if file.read_exact(&mut buf).is_err() {
            return Err(ReadError::IoError);
        }

        let mut extra_data = ExtraData::default();
        let mut num_colors = MAX_COLORS;
        if has_extra {
            num_colors = file.read_u16::<LittleEndian>().map_err(|_| ReadError::IoError)? as usize;
            extra_data.num_colors = num_colors as u16;
            extra_data.transparent_index = file.read_u16::<LittleEndian>().map_err(|_| ReadError::IoError)?;
        }

        let mut colors = Vec::with_capacity(num_colors);
        let mut offset = 0;
        let mut color = 0;
        while color < num_colors {
            colors.push(Srgb::new(buf[offset], buf[offset + 1], buf[offset + 2]));
            offset += 3;
            color += 1;
        }

        if has_extra {
            return Ok(Palette {
                colors,
                extra_data: Some(extra_data),
            });
        }

        Ok(Palette {
            colors,
            extra_data: None,
        })
    }
}

fn has_extra_data(file: &File) -> Result<bool, ReadError> {
    let file_size = file.metadata()?.len();
    if file_size == MAX_FILE_LENGTH_WITH_EXTRA_DATA as u64 {
        return Ok(true);
    } else if file_size != MAX_FILE_LENGTH as u64 {
        return Err(ReadError::InvalidFileLength);
    }
    Ok(false)
}


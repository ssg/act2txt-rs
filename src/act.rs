use std::io::Read;

use palette::Srgb;

pub const MAX_COLOR_BUFFER_LENGTH: usize = 768;
pub const MAX_COLORS: usize = 256;
pub const EXTRA_DATA_SIZE: usize = 4;

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
    pub fn read(reader: &mut dyn Read, all: bool) -> Result<Palette, ReadError> {
        let mut buf = [0; MAX_COLOR_BUFFER_LENGTH];
        reader.read_exact(&mut buf).map_err(|_| ReadError::InvalidFileLength)?;

        let (extra_data, num_colors) = fun_name(reader, all);

        let colors = buf
            .chunks_exact(3)
            .take(num_colors)
            .map(|chunk| Srgb::new(chunk[0], chunk[1], chunk[2]))
            .collect();

        Ok(Palette { colors, extra_data })
    }
}

fn fun_name(reader: &mut dyn Read, all: bool) -> (Option<ExtraData>, usize) {
    let mut extra_buf = [0; EXTRA_DATA_SIZE];
    let mut extra_data = None;
    let mut num_colors = MAX_COLORS;
    if reader.read_exact(&mut extra_buf).is_ok() {
        if !all {
            num_colors = u16::from_le_bytes([extra_buf[0], extra_buf[1]]) as usize;
        }
        extra_data = Some(ExtraData {
            num_colors: num_colors as u16,
            transparent_index: u16::from_le_bytes([extra_buf[2], extra_buf[3]]),
        });            
    }
    (extra_data, num_colors)
}
pub const MAGIC: u32 = 0xd00d_feed;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeviceTreeHeader {
    pub total_size: usize,

    pub structure_size: usize,
    pub strings_size: usize,

    pub structure_offset: usize,
    pub strings_offset: usize,

    pub reservation_offset: usize,
}

impl DeviceTreeHeader {
    pub const SIZE: usize = 40;

    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() < Self::SIZE {
            return Err(Error::TooSmall);
        }

        let magic = read_u32(bytes, 0);

        if magic != MAGIC {
            return Err(Error::InvalidMagic);
        }

        let header = Self {
            total_size: read_u32(bytes, 4) as usize,

            structure_size: read_u32(bytes, 36) as usize,
            strings_size: read_u32(bytes, 32) as usize,

            structure_offset: read_u32(bytes, 8) as usize,
            strings_offset: read_u32(bytes, 12) as usize,

            reservation_offset: read_u32(bytes, 16) as usize,
        };

        if header.total_size < Self::SIZE {
            return Err(Error::InvalidSize);
        }

        if !range_is_valid(
            header.structure_offset,
            header.structure_size,
            header.total_size,
        ) {
            return Err(Error::InvalidSize);
        }

        if !range_is_valid(
            header.strings_offset,
            header.strings_size,
            header.total_size,
        ) {
            return Err(Error::InvalidSize);
        }

        if header.reservation_offset >= header.total_size {
            return Err(Error::InvalidSize)
        }

        Ok(header)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    TooSmall,
    InvalidMagic,
    InvalidSize,
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}

fn range_is_valid(offset: usize, size: usize, total_size: usize) -> bool {
    match offset.checked_add(size) {
        Some(end) => end <= total_size,
        None => false,
    }
}

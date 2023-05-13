use std::fs::File;
use std::io::{Error, SeekFrom};
use std::io::prelude::*;

pub struct FirstSector {
  pub image: [u8; 512],
}

pub fn read_first_sector(f: &mut File) -> Result<FirstSector, Error> {
  let mut sector: [u8; 512] = [0; 512];
  f.seek(SeekFrom::Start(0))?;
  f.read_exact(&mut sector)?;

  Ok(FirstSector { image: sector })
}

pub fn oem_name(sector: &FirstSector) -> &str {
  let mem = &sector.image[3..9];
  std::str::from_utf8(mem).unwrap()
}

pub fn bytes_per_sector(sector: &FirstSector) -> u32 {
  (sector.image[0x00B] as u32)
  + ((sector.image[0x00B+1] as u32)  << 8)
}

pub fn sectors_per_cluster(sector: &FirstSector) -> u32 {
  sector.image[0x00D] as u32
}

pub fn reserved_sectors(sector: &FirstSector) -> u32 {
  (sector.image[0x00E] as u32)
  + ((sector.image[0x00E+1] as u32) << 8)
}

pub fn num_of_fats(sector: &FirstSector) -> u32 {
  // almost always 2
  sector.image[0x010] as u32
}

pub fn num_of_root_directory_entries(sector: &FirstSector) -> u32 {
  (sector.image[0x011] as u32)
  + ((sector.image[0x011+1] as u32) << 8)
}

pub fn total_sectors(sector: &FirstSector) -> u32 {
  let f16 = (sector.image[0x013] as u32) + ((sector.image[0x013+1] as u32) << 8);
  if f16 != 0 {
    f16
  } else {
    (sector.image[0x020] as u32)
    + ((sector.image[0x020+1] as u32) << 8)
    + ((sector.image[0x020+2] as u32) << 16)
    + ((sector.image[0x020+3] as u32) << 24)
  }
}

pub fn media_descriptor(sector: &FirstSector) -> u32 {
  sector.image[0x015] as u32
}

pub fn sectors_per_fats(sector: &FirstSector) -> u32 {
  let f16 = (sector.image[0x016] as u32) + ((sector.image[0x016] as u32) << 8);
  if f16 != 0 {
    f16
  } else {
    (sector.image[0x024] as u32)
    + ((sector.image[0x024+1] as u32) << 8)
    + ((sector.image[0x024+2] as u32) << 16)
    + ((sector.image[0x024+3] as u32) << 24)
  }
}

fn slice_reserved_sectors(sector: &FirstSector) -> &[u8] {
  let sector_size = bytes_per_sector(sector) as usize;
  let reserved_sector_count = reserved_sectors(sector) as usize;

  &sector.image[0..=(sector_size * reserved_sector_count)]
}

pub fn reserved_sector_size(sector: &FirstSector) -> u32 {
  bytes_per_sector(sector) * reserved_sectors(sector)
}

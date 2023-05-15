use std::fs::File;
use std::io::{Error, SeekFrom};
use std::io::prelude::*;

pub struct FirstSector {
  pub image: Vec<u8>, //[u8; 512],
}


impl FirstSector {

  pub fn read(f: &mut File) -> Result<FirstSector, Error> {
    let mut sector: [u8; 512] = [0; 512];
    f.seek(SeekFrom::Start(0))?;
    f.read_exact(&mut sector)?;

    Ok(FirstSector { image: sector.to_vec() })
  }

  pub fn oem_name(&self) -> &str {
    let mem = &self.image[3..9];
    std::str::from_utf8(mem).unwrap()
  }

  pub fn bytes_per_sector(&self) -> u32 {
    (self.image[0x00B] as u32)
    + ((self.image[0x00B+1] as u32)  << 8)
  }

  pub fn sectors_per_cluster(&self) -> u32 {
    self.image[0x00D] as u32
  }

  pub fn reserved_sectors(&self) -> u32 {
    (self.image[0x00E] as u32)
    + ((self.image[0x00E+1] as u32) << 8)
  }

  pub fn num_of_fats(&self) -> u32 {
    // almost always 2
    self.image[0x010] as u32
  }

  pub fn num_of_root_directory_entries(&self) -> u32 {
    (self.image[0x011] as u32)
    + ((self.image[0x011+1] as u32) << 8)
  }

  pub fn total_sectors(&self) -> u32 {
    let f16 = (self.image[0x013] as u32) + ((self.image[0x013+1] as u32) << 8);
    if f16 != 0 {
      f16
    } else {
      (self.image[0x020] as u32)
      + ((self.image[0x020+1] as u32) << 8)
      + ((self.image[0x020+2] as u32) << 16)
      + ((self.image[0x020+3] as u32) << 24)
    }
  }

  pub fn media_descriptor(&self) -> u32 {
    self.image[0x015] as u32
  }

  pub fn sectors_per_fats(&self) -> u32 {
    let f16 = (self.image[0x016] as u32) + ((self.image[0x016] as u32) << 8);
    if f16 != 0 {
      f16
    } else {
      (self.image[0x024] as u32)
      + ((self.image[0x024+1] as u32) << 8)
      + ((self.image[0x024+2] as u32) << 16)
      + ((self.image[0x024+3] as u32) << 24)
    }
  }

  pub fn slice_reserved_sectors(&self) -> &[u8] {
    let sector_size = self.bytes_per_sector() as usize;
    let reserved_sector_count = self.reserved_sectors() as usize;

    &self.image[0..=(sector_size * reserved_sector_count)]
  }

  pub fn reserved_sector_size(&self) -> u32 {
    self.bytes_per_sector() * self.reserved_sectors()
  }
}

use crate::bootsector;

pub enum FatType {
  FAT12,
  FAT16,
  FAT32
}

impl std::fmt::Display for FatType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      FatType::FAT12 => "FAT12",
      FatType::FAT16 => "FAT16",
      FatType::FAT32 => "FAT32",
      _ => "unknown"
    })
  }
}

pub struct Region {
  pub fat_start_sector: u32,
  pub fat_sector_size: u32,

  pub root_dir_start_sector: u32,
  pub root_dir_sector_size: u32,

  pub data_start_sector: u32,
  pub data_sector_size: u32,

  pub fat_type: FatType,

  pub sector_size: u32,
}

fn fat_type(data_sector_size: u32, sectors_per_cluster: u32) -> FatType {
  let count_of_clusters = data_sector_size / sectors_per_cluster;
  match count_of_clusters {
    n if 0 < n && n <= 4085 => FatType::FAT12,
    n if 4086 <= n && n <= 65525 => FatType::FAT16,
    _ => FatType::FAT32
  }
}


impl Region {
  pub fn new(sector: &bootsector::FirstSector) -> Self {
    let fat_start_sector = bootsector::reserved_sectors(sector);
    let fat_sector_size = bootsector::sectors_per_fats(sector) * bootsector::num_of_fats(sector);
    let root_dir_start_sector = fat_start_sector + fat_sector_size;
    let root_dir_sector_size = (32 * bootsector::num_of_root_directory_entries(sector) + bootsector::bytes_per_sector(sector) - 1) / bootsector::bytes_per_sector(sector);
    let data_start_sector = root_dir_start_sector + root_dir_sector_size;
    let data_sector_size = bootsector::total_sectors(sector) - data_start_sector;
    let fat_type = fat_type(data_sector_size, bootsector::sectors_per_cluster(sector));
    let sector_size = bootsector::bytes_per_sector(sector);

    Self {
      fat_start_sector,
      fat_sector_size,
      root_dir_start_sector,
      root_dir_sector_size,
      data_start_sector,
      data_sector_size,
      fat_type,
      sector_size
    }
  }
}

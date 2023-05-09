mod bootsector;
mod fat;

fn main() {
    let image = std::fs::read("data.bin").unwrap();
    println!("{}", image.len());

    println!("oem_name:{}", bootsector::oem_name(&image));
    println!("total_sectors:{}", bootsector::total_sectors(&image));

    let fat = fat::Region::new(&image);
    println!("type:{}", fat.fat_type);

    println!("dir:{}", fat.root_dir_start_sector);

    println!("{:?}", &image[((fat.root_dir_start_sector * fat.sector_size) as usize)..]);

}


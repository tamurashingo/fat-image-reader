mod bootsector;
mod fat;
mod dir;


fn main() {
    let image = std::fs::read("data.bin").unwrap();
    println!("{}", image.len());

    //let image = std::fs::read("data.bin").unwrap();
    let mut f = std::fs::File::open("data.bin").unwrap();
    let first_sector = bootsector::FirstSector::read(&mut f).unwrap();
    println!("1st sector:{:?}", first_sector.image);

    println!("oem_name:{}", first_sector.oem_name());

    let fat = fat::Region::new(&first_sector);
    let directory_entries = dir::DirectoryEntry::new(
        &image[((fat.root_dir_start_sector * fat.sector_size) as usize)..(((fat.root_dir_start_sector + fat.root_dir_sector_size) * fat.sector_size) as usize)].to_vec()
    );

    for d in directory_entries {
        println!("----------");
        println!("filename: {}.{}", d.filename(), d.extension());
        println!("create  : {} {}", d.created_date(), d.created_time());
        println!("update  : {} {}", d.updated_date(), d.updated_time());
    }


}




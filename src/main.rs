mod bootsector;

fn main() {
    let image = std::fs::read("data.bin").unwrap();
    println!("{}", image.len());

    println!("oem_name:{}", bootsector::oem_name(&image));
    println!("total_sectors:{}", bootsector::total_sectors(&image));

}


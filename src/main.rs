#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct BootSector {
    BS_JmpBoot: [u8; 3],
    BS_OEMName: [u8; 8],
    BPB_BytsPerSec: [u8; 2],
    BPB_SecPerClus: [u8; 1],
    BPB_RsvdSecCnt: [u8; 2],
    BPB_NumFATs: [u8; 1],
    BPB_RootEntCnt: [u8; 2],
    BPB_TotSec16: [u8; 2],
    BPB_Media: [u8; 1],
    BPB_FATSz16: [u8; 2],
    BPB_SecPerTrk: [u8; 2],
    BPB_NumHeads: [u8; 2],
    BPB_HiddSec: [u8; 4],
    BPB_TotSec32: [u8; 4],


    BS_DrvNum: [u8; 1],
    BS_Reserved: [u8; 1],
    BS_BootSig: [u8; 1],
    BS_VolID: [u8; 4],
    BS_VolLab: [u8; 11],
    BS_FilSysType: [u8; 8],
    BS_BootCode: [u8; 448],
    BS_BootSign: [u8; 2],
}

impl BootSector {
    fn BS_OEMName(&self) -> &str {
        std::str::from_utf8(&self.BS_OEMName).unwrap()
    }

    fn BPB_BytsPerSec(&self) -> u32 {
        (self.BPB_BytsPerSec[0] as u32) + ((self.BPB_BytsPerSec[1] as u32) << 8)
    }
}


fn main() {
    let bytes = std::fs::read("data.bin").unwrap();
    println!("{}", bytes.len());

    let (head, body, _tail) = unsafe { bytes.align_to::<BootSector>() };
    let boot_sector = &body[0];

    println!("{:?}", boot_sector);

    println!("OEMName:{}", boot_sector.BS_OEMName());
    println!("BytsPerSec:{}", boot_sector.BPB_BytsPerSec());
}


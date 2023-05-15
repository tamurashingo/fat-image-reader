

pub struct FileAttribute {
  attribute: u8,
}

impl FileAttribute {
  pub fn new(attribute: u8) -> FileAttribute {
    FileAttribute { attribute: attribute }
  }

  pub fn isReadOnly(&self) -> bool {
    self.attribute & 0x01 != 0x00
  }

  pub fn isHidden(&self) -> bool {
    self.attribute & 0x02 != 0x00
  }

  pub fn isSystem(&self) -> bool {
    self.attribute & 0x04 != 0x00
  }

  pub fn isVolume(&self) -> bool {
    self.attribute & 0x08 != 0x00
  }

  pub fn isDirectory(&self) -> bool {
    self.attribute & 0x10 != 0x00
  }

  pub fn isArchive(&self) -> bool {
    self.attribute & 0x20 != 0x00
  }
}

pub struct Time {
  hhmmss: u16
}

impl Time {
  pub fn new(hhmmss: u16) -> Time {
    Time { hhmmss: hhmmss }
  }

  /// ```
  /// use fat_image_reader::dir::Time;
  /// 
  /// let t = Time::new(0x645C);
  /// assert_eq!(t.hour(), 12)
  /// ```
  pub fn hour(&self) -> u8 {
    (((self.hhmmss & 0xF800) >> 11) & 0xFF) as u8
  }

  /// ```
  /// use fat_image_reader::dir::Time;
  /// 
  /// let t = Time::new(0x645C);
  /// assert_eq!(t.minute(), 34);
  /// ```
  pub fn minute(&self) -> u8 {
    (((self.hhmmss & 0x07E0) >> 5) & 0xFF) as u8
  }

  /// ```
  /// use fat_image_reader::dir::Time;
  /// 
  /// let t = Time::new(0x645C);
  /// assert_eq!(t.second(), 56);
  /// ```
  pub fn second(&self) -> u8 {
    ((self.hhmmss & 0x001F) * 2) as u8
  }
}

pub struct Date {
  yymmdd: u16
}

impl Date {
  pub fn new(yymmdd: u16) -> Date {
    Date { yymmdd: yymmdd }
  }

  pub fn year(&self) -> u16 {
    ((self.yymmdd & 0xFE00) >> 9) + 1980
  }
}

pub struct DirectoryEntry {
  pub bytes: Vec<u8>, //&'a [u8; 32],
  file_attribute: FileAttribute,
  update_time: Time,
  update_date: Date,
}

impl DirectoryEntry {
  pub fn new(raw: &Vec<u8>) -> DirectoryEntry {
    //&raw[0..32];
    //let bytes: &[u8; 32] = raw[0..32].collect();
    let bytes = raw[0..32].to_vec();
    let file_attribute = FileAttribute::new(bytes[0x0B]);
    let update_time = Time::new((bytes[0x16] as u16) + (bytes[0x16+1] as u16) << 8);
    let update_date = Date::new((bytes[0x18] as u16) + (bytes[0x18+1] as u16) << 8);
    DirectoryEntry {
      bytes: bytes,
      file_attribute: file_attribute,
      update_time: update_time,
      update_date: update_date
    }
  }



  pub fn filename(&self) -> &str {
    // TODO: 全角文字への対応
    let mem: &[u8] = &self.bytes[0..9];
    std::str::from_utf8(&mem).unwrap()
  }

  pub fn extension(&self) -> &str {
    // TODO: 全角文字への対応
    let mem: &[u8] = &self.bytes[9..12];
    std::str::from_utf8(mem).unwrap()
  }

  pub fn file_attribute(&self) -> &FileAttribute {
    &self.file_attribute
  }

  pub fn update_time(&self) -> &Time {
    &self.update_time
  }

  pub fn update_date(&self) -> &Date {
    &self.update_date
  }
}


// #[test]
// pub fn test_hour() {
//   assert_eq!(hour(0x6490), 12);
// }

// pub fn minute(time: u16) -> u8 {
//   (((time & 0x07E0) >> 5) & 0xFF) as u8
// }

// #[test]
// pub fn test_minute() {
//   assert_eq!(minute(0x6490), 36);
// }

// pub fn second(time: u16) -> u8 {
//   (((time & 0x001F) & 0xFF) as u8)
// }

// #[test]
// pub fn test_second() {
//   assert_eq!(second(0x6490), 16);
// }



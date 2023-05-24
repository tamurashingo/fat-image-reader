use encoding_rs::{self, SHIFT_JIS};
use std::borrow::Cow;

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

impl std::fmt::Display for Time {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:02}:{:02}:{:02}", self.hour(), self.minute(), self.second())
  }
}

pub struct Date {
  yymmdd: u16
}

impl Date {
  pub fn new(yymmdd: u16) -> Date {
    Date { yymmdd: yymmdd }
  }

  /// ```
  /// use fat_image_reader::dir::Date;
  /// 
  /// let d = Date::new(0x3281);
  /// assert_eq!(d.year(), 2005)
  /// ```
  pub fn year(&self) -> u16 {
    ((self.yymmdd & 0xFE00) >> 9) + 1980
  }

  /// ```
  /// use fat_image_reader::dir::Date;
  /// 
  /// let d = Date::new(0x3281);
  /// assert_eq!(d.month(), 4)
  /// ```
  pub fn month(&self) -> u8 {
    (((self.yymmdd & 0x01E0) >> 5) & 0xFF) as u8
  }

  /// ```
  /// use fat_image_reader::dir::Date;
  /// 
  /// let d = Date::new(0x3281);
  /// assert_eq!(d.day(), 1)
  /// ```
  pub fn day(&self) -> u8 {
    (self.yymmdd & 0x001F) as u8
  }
}

impl std::fmt::Display for Date {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:04}/{:02}/{:02}", self.year(), self.month(), self.day())
  }
}


pub struct DirectoryEntry {
  pub bytes: Vec<u8>, //&'a [u8; 32],
  file_attribute: FileAttribute,
  created_time: Time,
  created_date: Date,
  updated_time: Time,
  updated_date: Date,
  pub directory: bool,
  pub deleted: bool,
}

impl DirectoryEntry {
  /// root_directory_entryのメモリ情報からディレクトリ情報を取得する
  /// 
  /// * `raw` - root_directory_entryのメモリ情報
  pub fn new(raw: &Vec<u8>) -> Vec<DirectoryEntry> {
    let entries = raw.len() / 32;
    let mut dirs: Vec<DirectoryEntry> = Vec::with_capacity(entries);


    for entry in 0..entries {
      // ファイル名の先頭が\0はこれ以降が無効なエントリとなる
      if raw[entry * 32] == 0 {
        break;
      }
      dirs.push(DirectoryEntry::new_inner(&raw, entry));
    }

    dirs
  }


  /// 指定されたroot_directory_entryのメモリ情報からディレクトリ情報を切り出す
  /// 
  /// * `raw` - root_directoryのメモリ情報
  /// * `entry` - ディレクトリ構造体の位置
  fn new_inner(raw: &Vec<u8>, entry: usize) -> DirectoryEntry {
    let bytes = raw[(entry * 32)..(entry * 32 + 32)].to_vec();
    let file_attribute = FileAttribute::new(bytes[0x0B]);
    let created_time = Time::new((bytes[0x0E] as u16) + ((bytes[0x0E+1] as u16) << 8));
    let created_date = Date::new((bytes[0x10] as u16) + ((bytes[0x10+1] as u16) << 8));
    let updated_time = Time::new((bytes[0x16] as u16) + ((bytes[0x16+1] as u16) << 8));
    let updated_date = Date::new((bytes[0x18] as u16) + ((bytes[0x18+1] as u16) << 8));
    let directory = bytes[0x1C] == 0 && bytes[0x1C+1] == 0 && bytes[0x1C+2] == 0 && bytes[0x1C+3] == 0;
    let deleted = bytes[0] == 0x00;
    DirectoryEntry {
      bytes,
      file_attribute,
      created_time,
      created_date,
      updated_date,
      updated_time,
      directory,
      deleted,
    }
  }

  pub fn filename(&self) -> Cow<str> {
    let mut pos = 0;
    for idx in 0..8 {
      pos = idx;
      if self.bytes[idx] == 0x20 {
        break;
      }
    }

    // TODO: デコードエラーへの対応
    // TODO: 使用できない文字への対応
    let mem: &[u8] = &self.bytes[0..pos];
    let (cow, encoding_used, had_errors) = SHIFT_JIS.decode(&mem);
    cow
  }

  pub fn extension(&self) -> Cow<str> {
    let mut pos = 8;
    for idx in 8..12 {
      pos = idx;
      if self.bytes[idx] == 0x20 {
        break;
      }
    }
    // TODO: デコードエラーへの対応
    // TODO: 使用できない文字への対応
    let mem: &[u8] = &self.bytes[8..pos];
    let (cow, encoding_used, had_errors) = SHIFT_JIS.decode(&mem);
    cow
  }

  pub fn file_attribute(&self) -> &FileAttribute {
    &self.file_attribute
  }

  pub fn created_time(&self) -> &Time {
    &self.created_time
  }

  pub fn created_date(&self) -> &Date {
    &self.created_date
  }

  pub fn updated_time(&self) -> &Time {
    &self.updated_time
  }

  pub fn updated_date(&self) -> &Date {
    &self.updated_date
  }

  pub fn filesize(&self) -> u32 {
    (self.bytes[0x1C] as u32)
    + ((self.bytes[0x1C+1] as u32) << 8)
    + ((self.bytes[0x1C+2] as u32) << 16)
    + ((self.bytes[0x1C+3] as u32) << 24)
  }
}






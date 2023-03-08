use std::fs::File;
use std::io::Write;
use std::ops::Index;

#[derive(Debug)]
pub struct KeyOffset {
    pub key: String,
    pub offset: u64,
}

fn write_index() {
    let index = File::create(".glossary/index.bin").expect("could not create file");
    let top_level_index = File::create(".glossary/top_index.bin").expect("could not create file");

    let mut writer = std::io::BufWriter::new(index);
    let mut top_level_writer = std::io::BufWriter::new(top_level_index);

    let mut top_level_offset: usize = 0;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(".glossary/sorted_offsets.csv")
        .expect("could not read sorted offsets");
    for row in reader.records() {
        let record = row.expect("could not parse csv");

        let o = KeyOffset {
            key: record.index(0).to_owned(),
            offset: record.index(1).parse::<u64>().unwrap(),
        };

        let key_bytes = o.key.as_bytes();
        let num_key_bytes = key_bytes.len();

        top_level_writer
            .write(&num_key_bytes.to_le_bytes())
            .expect("failed to write key length");
        top_level_writer
            .write(&top_level_offset.to_le_bytes())
            .expect("failed to write key offset");

        // key byte length plus 8 bytes for usize
        top_level_offset += num_key_bytes + 8;

        writer.write(key_bytes).expect("failed to write key");
        writer
            .write(&o.offset.to_le_bytes())
            .expect("failed to write offset");
    }

    writer.flush().expect("failed to flush writer");
    top_level_writer.flush().expect("failed to flush writer");
}

pub fn generate_index(f: &mut File, key_index: usize, delimiter: char) {
    std::fs::create_dir_all(".glossary").expect("could not create dir");
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .terminator(csv::Terminator::Any(b'\n'))
        .from_reader(f);

    let mut cmd = std::process::Command::new("sort")
        .arg("-o")
        .arg(".glossary/sorted_offsets.csv")
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn sort command");

    let mut csv_writer =
        csv::Writer::from_writer(cmd.stdin.as_mut().expect("could not pipe data to sort"));

    for row in reader.byte_records() {
        let record = row.expect("could not parse csv");
        let pos = record.position().unwrap();
        let key = record.index(key_index).to_owned();

        csv_writer
            .write_record(&[
                std::str::from_utf8(&key).unwrap().to_owned(),
                pos.byte().to_string(),
            ])
            .expect("could not write record");
    }

    csv_writer.flush().expect("could not flush csv");
    drop(csv_writer);

    cmd.wait().expect("sort failed");

    write_index();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufRead;
    use std::io::Read;
    use std::io::Seek;
    #[test]
    fn test_generate_index() {
        let mut f = File::open("MOCK_DATA.csv").expect("count not find data file");
        generate_index(&mut f, 3, ',');

        // test top level index
        let mut top_index = File::open(".glossary/top_index.bin").unwrap();

        let mut length_bytes = [0; 8];
        let mut offset_bytes = [0; 8];
        top_index.read_exact(&mut length_bytes).unwrap();
        top_index.read_exact(&mut offset_bytes).unwrap();
        let length = u64::from_le_bytes(length_bytes);
        let offset = u64::from_le_bytes(offset_bytes);

        // first key is aarndtsenk6@marriott.com which is 24 bytes
        assert_eq!(length, 24);
        assert_eq!(offset, 0);

        // test index
        let mut index = File::open(".glossary/index.bin").unwrap();
        let mut key_bytes = vec![0u8; length as usize];
        let mut offset_bytes = [0; 8];

        index
            .read_exact(&mut key_bytes)
            .expect("failed to read key from index");

        index
            .read_exact(&mut offset_bytes)
            .expect("failed to read offset from index");

        let offset = u64::from_le_bytes(offset_bytes);

        assert_eq!(
            std::str::from_utf8(&key_bytes).expect("could not convert to str"),
            "aarndtsenk6@marriott.com"
        );
        assert_eq!(offset, 44971);

        // test data access
        let mut f = File::open("MOCK_DATA.csv").expect("count not find data file");
        f.seek(std::io::SeekFrom::Start(offset))
            .expect("could not seek file");
        let mut reader = std::io::BufReader::new(f);
        let mut row = std::string::String::new();
        reader.read_line(&mut row).expect("could not read row");

        assert_eq!(
            row,
            "727,Antone,Arndtsen,aarndtsenk6@marriott.com,Male,251.8.128.77\n"
        )
    }
}

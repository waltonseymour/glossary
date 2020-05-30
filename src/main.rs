use std::fs::File;
use std::io::BufRead;
use std::io::Seek;
use std::io::Write;
use std::ops::Index;

struct KeyOffset {
    key: String,
    offset: u64,
}

fn generate_index(f: File) {
    std::fs::create_dir_all(".indexer").expect("could not create dir");
    let index = File::create(".indexer/index.bin").expect("could not create file");
    let top_level_index = File::create(".indexer/top_index.bin").expect("could not create file");

    let mut writer = std::io::BufWriter::new(index);
    let mut top_level_writer = std::io::BufWriter::new(top_level_index);
    let mut reader = csv::Reader::from_reader(f);

    let mut offsets = std::vec::Vec::<KeyOffset>::new();

    for row in reader.records() {
        let record = row.unwrap();
        let pos = record.position().unwrap();
        let key = record.index(3).to_owned();
        offsets.push(KeyOffset {
            key: key,
            offset: pos.byte(),
        });
    }

    offsets.sort_by(|a, b| a.key.partial_cmp(&b.key).unwrap());

    let mut top_level_offset: usize = 0;

    for o in offsets {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    #[test]
    fn test_generate_index() {
        let f = File::open("MOCK_DATA.csv").expect("count not find data file");
        generate_index(f);

        // test top level index
        let mut top_index = File::open(".indexer/top_index.bin").unwrap();

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
        let mut index = File::open(".indexer/index.bin").unwrap();
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

fn main() {}

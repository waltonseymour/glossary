use std::fs::File;
use std::io::Write;
use std::ops::Index;

struct KeyOffset {
    key: String,
    offset: u64,
}

fn generate_index(f: File) {
    std::fs::create_dir_all(".indexer").expect("could not create dir");
    let index = File::create(".indexer/index.bin").unwrap();
    let mut writer = std::io::BufWriter::new(index);
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

    for o in offsets {
        writer.write(o.key.as_bytes()).expect("failed to write key");
        writer
            .write(&o.offset.to_le_bytes())
            .expect("failed to write offset");
    }

    writer.flush().expect("failed to flush writer");
}

fn main() {
    let f = File::open("MOCK_DATA.csv").expect("count not find data file");

    generate_index(f);
}

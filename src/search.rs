use std::fs::File;
use std::io::Read;
use std::io::Seek;

use super::write::KeyOffset;

fn get_nth_element(top_index: &mut File, index: &mut File, n: u64) -> KeyOffset {
    // direct access to nth entry in top level index
    top_index
        .seek(std::io::SeekFrom::Start(n * 16))
        .expect("could not seek file");

    let mut length_bytes = [0; 8];
    let mut offset_bytes = [0; 8];
    top_index.read_exact(&mut length_bytes).unwrap();
    top_index.read_exact(&mut offset_bytes).unwrap();

    // pull length of key and offset in index.bin
    let length = u64::from_le_bytes(length_bytes);
    let offset = u64::from_le_bytes(offset_bytes);

    // seek index.bin to offset
    index
        .seek(std::io::SeekFrom::Start(offset))
        .expect("could not seek file");

    // read key and offset
    let mut key_bytes = vec![0u8; length as usize];
    let mut offset_bytes = [0; 8];
    index
        .read_exact(&mut key_bytes)
        .expect("failed to read key from index");
    index
        .read_exact(&mut offset_bytes)
        .expect("failed to read offset from index");
    let offset = u64::from_le_bytes(offset_bytes);

    KeyOffset {
        key: std::str::from_utf8(&key_bytes)
            .expect("failed to unmarshal key")
            .to_owned(),
        offset: offset,
    }
}

pub fn search(top_index: &mut File, index: &mut File, key: String) -> Option<KeyOffset> {
    let total_elements = top_index.metadata().expect("could not get metadata").len() / 16;

    let mut min = 0;
    let mut max = total_elements - 1;

    while max - min > 0 {
        let midpoint = min + ((max - min) / 2);

        let offset = get_nth_element(top_index, index, midpoint);

        if offset.key == key {
            return Some(offset);
        } else if offset.key.partial_cmp(&key).unwrap() == std::cmp::Ordering::Less {
            min = midpoint;
        } else {
            max = midpoint;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::super::write::generate_index;
    use super::*;
    #[test]
    fn test_search() {
        let f = File::open("MOCK_DATA.csv").expect("count not find data file");
        generate_index(f);

        let mut index = std::fs::File::open(".glossary/index.bin").expect("failed to open index");
        let mut top_index =
            std::fs::File::open(".glossary/top_index.bin").expect("failed to open top index");

        let offset = search(
            &mut top_index,
            &mut index,
            String::from("ckamanek@jimdo.com"),
        )
        .unwrap();

        assert_eq!(offset.key, "ckamanek@jimdo.com");
        assert_eq!(offset.offset, 32490)
    }
}

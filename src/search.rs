use std::fs::File;
use std::io::BufRead;
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

fn search(top_index: &mut File, index: &mut File, key: String) -> Option<KeyOffset> {
    let total_elements = top_index.metadata().expect("could not get metadata").len() / 16;

    let mut min = 0;
    let mut max = total_elements;

    let mut size: u64 = total_elements;
    while size > 0 {
        let midpoint = min + ((max - min) / 2);
        size /= 2;

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

pub fn find_key(key: String) -> Option<KeyOffset> {
    let mut index = std::fs::File::open(".glossary/index.bin").expect("failed to open index");
    let mut top_index =
        std::fs::File::open(".glossary/top_index.bin").expect("failed to open top index");

    search(&mut top_index, &mut index, key)
}

pub fn get_matching_row(data_file: &mut File, key: String) -> Option<String> {
    let offset = find_key(key);
    match offset {
        Some(x) => {
            data_file
                .seek(std::io::SeekFrom::Start(x.offset))
                .expect("could not seek file");
            let mut reader = std::io::BufReader::new(data_file);
            let mut row = std::string::String::new();
            reader.read_line(&mut row).expect("could not read row");
            Some(row)
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::super::write::generate_index;
    use super::*;

    #[test]
    fn test_get_matching_row() {
        let mut f = File::open("MOCK_DATA.csv").expect("count not find data file");
        generate_index(&mut f, 3, b',');

        let row = get_matching_row(&mut f, String::from("ckamanek@jimdo.com")).unwrap();

        assert_eq!(
            row,
            "525,Corrinne,Kaman,ckamanek@jimdo.com,Female,23.16.105.124\n"
        );

        let row = get_matching_row(&mut f, String::from("aarndtsenk6@marriott.com")).unwrap();
        assert_eq!(
            row,
            "727,Antone,Arndtsen,aarndtsenk6@marriott.com,Male,251.8.128.77\n"
        );

        let row = get_matching_row(&mut f, String::from("zsparksoz@twitter.com")).unwrap();
        assert_eq!(
            row,
            "900,Zonnya,Sparks,zsparksoz@twitter.com,Female,248.252.234.26\n"
        );
    }

    #[test]
    fn test_search() {
        let mut f = File::open("MOCK_DATA.csv").expect("count not find data file");
        generate_index(&mut f, 3, b',');

        let offset = find_key(String::from("ckamanek@jimdo.com")).unwrap();

        assert_eq!(offset.key, "ckamanek@jimdo.com");
        assert_eq!(offset.offset, 32490);

        let offset = find_key(String::from("aarndtsenk6@marriott.com")).unwrap();
        assert_eq!(offset.key, "aarndtsenk6@marriott.com");
        assert_eq!(offset.offset, 44971);

        let offset = find_key(String::from("zsparksoz@twitter.com")).unwrap();
        assert_eq!(offset.key, "zsparksoz@twitter.com");
        assert_eq!(offset.offset, 55806);
    }

    #[test]
    fn test_bad_search() {
        let mut f = File::open("MOCK_DATA.csv").expect("count not find data file");
        generate_index(&mut f, 3, b',');

        let mut index = std::fs::File::open(".glossary/index.bin").expect("failed to open index");
        let mut top_index =
            std::fs::File::open(".glossary/top_index.bin").expect("failed to open top index");

        let offset = search(&mut top_index, &mut index, String::from("asdfasdfasd"));

        assert_eq!(offset.is_none(), true);
    }
}

use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

pub(crate) fn find_signal_start_in_file(path: &str, marker_size: usize) -> usize {
    let mut fd = File::open(path).unwrap();

    let mut contents = String::new();
    fd.read_to_string(&mut contents).unwrap();
    for (i, four) in contents.as_bytes().windows(marker_size).enumerate() {
        let set: HashSet<&u8> = HashSet::from_iter(four);
        if set.len() == marker_size {
            return marker_size + i;
        }
    }
    0
}

mod hostsmanager;

use hostsmanager::hostsfile::HostsFile;

fn main() {
    let mut hf = HostsFile::new();
    hf.load_from_file("/etc/hosts")
        .expect("Can't read from file");
    hf.block_name_www("test.com");
    hf.write_to_file("hosts").expect("Can't write to file");
    hf.list_entries();
}

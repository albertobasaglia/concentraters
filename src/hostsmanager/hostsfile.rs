use super::hostsentry::HostsEntry;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

pub struct HostsFile {
    entries: Vec<HostsEntry>,
}

impl HostsFile {
    pub fn new() -> Self {
        let hosts_file = HostsFile { entries: vec![] };
        hosts_file
    }

    pub fn add_entry(&mut self, entry: HostsEntry) {
        let mut found = false;
        for existing in &mut self.entries {
            if existing.ip == entry.ip {
                for name in &entry.names {
                    existing.names.push(name.clone());
                }
                found = true;
                break;
            }
        }
        if !found {
            self.entries.push(entry);
        }
    }

    pub fn add_easy_entry(&mut self, ip: &str, first_name: &str) {
        self.add_entry(HostsEntry::new(ip, first_name));
    }

    pub fn list_entries(&self) {
        for e in &self.entries {
            println!("Ip: {}", e.ip);
            for n in &e.names {
                println!("\t {}", n);
            }
        }
    }

    pub fn load_from_file(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let lines = reader.lines();
        for line in lines {
            let strline = line.expect("Can't read line!");
            if strline.starts_with("#") || strline.starts_with(" ") || strline.len() == 0 {
                // println!("found invalid line");
            } else {
                let newentry = HostsEntry::from_string(strline);
                self.add_entry(newentry);
            }
        }
        Ok(())
    }

    pub fn write_to_file(&self, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        for entry in &self.entries {
            let mut out = entry.ip.clone();
            out += " ";
            out += entry.names.join(" ").as_str();
            out += "\n";
            writer.write_all(out.as_bytes())?;
        }
        Ok(())
    }

    pub fn block_name(&mut self, name: &str) {
        self.add_easy_entry("127.0.0.1", name);
        self.add_easy_entry("::1", name);
    }

    pub fn block_name_www(&mut self, name: &str) {
        self.block_name(name);
        self.block_name(("www.".to_string() + name).as_str());
    }
}

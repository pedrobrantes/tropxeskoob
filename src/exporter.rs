use crate::models::SkoobBook;
use anyhow::Result;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufWriter;

pub struct SkoobExporter;

impl SkoobExporter {
    pub fn to_json(data: &[SkoobBook], file_path: &str) -> Result<()> {
        let file = File::create(file_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, data)?;
        println!("[+] JSON generated successfully: '{}'", file_path);
        Ok(())
    }

    pub fn to_csv(data: &[SkoobBook], file_path: &str) -> Result<()> {
        if data.is_empty() {
            println!("[!] No data to export to CSV.");
            return Ok(());
        }

        let mut all_keys = BTreeSet::new();
        for book in data {
            for key in book.extra.keys() {
                all_keys.insert(key.clone());
            }
        }

        let file = File::create(file_path)?;
        let mut writer = csv::Writer::from_writer(file);

        // Write header
        let headers: Vec<&String> = all_keys.iter().collect();
        writer.write_record(&headers)?;

        // Write rows
        for book in data {
            let mut row = Vec::new();
            for key in &all_keys {
                let value = book
                    .extra
                    .get(key)
                    .map(|v| match v {
                        serde_json::Value::String(s) => s.clone(),
                        _ => v.to_string(),
                    })
                    .unwrap_or_default();
                row.push(value);
            }
            writer.write_record(&row)?;
        }

        writer.flush()?;
        println!(
            "[+] CSV generated successfully: '{}' ({} columns identified)",
            file_path,
            all_keys.len()
        );
        Ok(())
    }
}

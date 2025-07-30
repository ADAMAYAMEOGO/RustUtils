// src/main.rs
use std::fs::{File, remove_file};
use std::io::{Write, Read};
use std::time::Instant;
use std::path::Path;

fn main() {
    let file_path = "test_benchmark.tmp";

    let data_size_mb = 20000;
    let data = vec![0u8; 1024 * 1024]; // 1 Mo = 1024 * 1024 octets

    let start_write = Instant::now();
    let mut file = File::create(file_path).expect("Failed to create file");
    for i in 0..data_size_mb {
        file.write_all(&data).expect("Failed to write data");
        if i % 1000 == 0 {
            println!("Writing progress: {} MB", i / 1000);
        }
    }
    let write_duration = start_write.elapsed();

    println!("✅ Writing finished in  {:?}", write_duration);

    let start_read = Instant::now();
    let mut file = File::open(file_path).expect("Failed to open file");
    for i in 0..data_size_mb {
        let mut buffer = vec![0u8; data.len()];
        file.read_exact(&mut buffer).expect("Failed to read data");
        if i % 1000 == 0 {
            println!("Reading progress: {} MB", i / 1000);
        }
    }
    let read_duration = start_read.elapsed();

    println!("✅ Read finished in {:?}", read_duration);

    let write_speed = data_size_mb as f64 / write_duration.as_secs_f64();
    let read_speed = data_size_mb as f64 / read_duration.as_secs_f64();

    println!("🚀 Write speed : {:.2} Mo/s", write_speed);
    println!("🚀 Read Speed  : {:.2} Mo/s", read_speed);

    if Path::new(file_path).exists() {
        remove_file(file_path).expect("Failed to delete file");
        println!("🧼 temp file deleted");
    }
} 

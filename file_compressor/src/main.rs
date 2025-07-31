// src/main.rs
use std::fs::{File, metadata};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::time::Instant;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }

    let input_path = &args[1];
    let output_path = format!("{}.gz", input_path);

    let start = Instant::now();

    let input_file = File::open(input_path).expect("❌ Impossible d'ouvrir le fichier source");
    let mut reader = BufReader::new(input_file);

    let output_file = File::create(&output_path).expect("❌ Impossible de créer le fichier compressé");
    let writer = BufWriter::new(output_file);

    let mut encoder = GzEncoder::new(writer, Compression::default());
    std::io::copy(&mut reader, &mut encoder).expect("❌ Échec de la compression");
    let writer = encoder.finish().expect("❌ Échec de la finalisation de la compression");

    let duration = start.elapsed();
    let original_size = metadata(input_path).unwrap().len();
    let compressed_size = metadata(&output_path).unwrap().len();

    println!("✅ Compression terminée en {:?}", duration);
    println!("📦 Taille d'origine  : {:.2} Ko", original_size as f64 / 1024.0);
    println!("📦 Taille compressée : {:.2} Ko", compressed_size as f64 / 1024.0);
    println!("🎯 Fichier compressé : {}", output_path);
}

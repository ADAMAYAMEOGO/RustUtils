use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::io::Read;
use anyhow::{anyhow, Context, Result};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

/// Build default output path for compression: <file>.gz (if no --out provided)
fn default_out_compress(input: &Path) -> PathBuf {
    let mut out = input.to_path_buf();
    out.set_extension(format!(
        "{}gz",
        input
            .extension()
            .map(|e| format!("{}.", e.to_string_lossy()))
            .unwrap_or_default()
    ));
    out
}

/// Build default output path for decompression: remove .gz (if no --out provided)
fn default_out_decompress(input: &Path) -> Result<PathBuf> {
    let ext = input.extension().and_then(|e| e.to_str()).unwrap_or("");
    if ext != "gz" {
        return Err(anyhow!(
            "Input file must end with .gz if no --out is provided"
        ));
    }
    let mut out = input.to_path_buf();
    out.set_extension("");
    if let Some(stem) = input.file_stem() {
        out.set_file_name(stem);
    }
    Ok(out)
}

/// Compress a file to gzip (streaming, no full memory load)
pub fn compress_file(input: &Path, out: Option<&Path>) -> Result<PathBuf> {
    if !input.exists() {
        return Err(anyhow!("Input file not found: {}", input.display()));
    }

    let output_path = out.map(|p| p.to_path_buf()).unwrap_or_else(|| default_out_compress(input));
    if input == output_path {
        return Err(anyhow!(
            "Input and output cannot be the same: {}",
            input.display()
        ));
    }

    let reader = BufReader::new(
        File::open(input).with_context(|| format!("Cannot open {}", input.display()))?,
    );

    let writer = BufWriter::new(
        File::create(&output_path)
            .with_context(|| format!("Cannot create {}", output_path.display()))?,
    );

    let mut encoder = GzEncoder::new(writer, Compression::default());
    io::copy(&mut reader.take(u64::MAX), &mut encoder)
        .context("Failed while copying data (compression)")?;
    encoder.try_finish().context("Failed to finalize gzip")?;

    Ok(output_path)
}

/// Decompress a gzip file (streaming)
pub fn decompress_file(input: &Path, out: Option<&Path>) -> Result<PathBuf> {
    if !input.exists() {
        return Err(anyhow!("Input file not found: {}", input.display()));
    }

    let output_path = match out {
        Some(p) => p.to_path_buf(),
        None => default_out_decompress(input)?,
    };
    if input == output_path {
        return Err(anyhow!(
            "Input and output cannot be the same: {}",
            input.display()
        ));
    }

    let reader = BufReader::new(
        File::open(input).with_context(|| format!("Cannot open {}", input.display()))?,
    );

    let mut decoder = GzDecoder::new(reader);

    let writer = BufWriter::new(
        File::create(&output_path)
            .with_context(|| format!("Cannot create {}", output_path.display()))?,
    );

    io::copy(&mut decoder, &mut std::io::BufWriter::new(writer))
        .context("Failed while copying data (decompression)")?;

    Ok(output_path)
}

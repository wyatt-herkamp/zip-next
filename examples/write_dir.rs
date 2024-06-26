use std::io::prelude::*;
use zip_next::result::ZipError;
use zip_next::write::FileOptions;

use std::fs::File;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn main() {
    std::process::exit(real_main());
}

const METHOD_STORED: Option<zip_next::CompressionMethod> =
    Some(zip_next::CompressionMethod::Stored);

#[cfg(any(
    feature = "deflate",
    feature = "deflate-miniz",
    feature = "deflate-zlib",
    feature = "deflate-zlib-ng"
))]
const METHOD_DEFLATED: Option<zip_next::CompressionMethod> =
    Some(zip_next::CompressionMethod::Deflated);
#[cfg(not(any(
    feature = "deflate",
    feature = "deflate-miniz",
    feature = "deflate-zlib",
    feature = "deflate-zlib-ng",
    feature = "deflate-zopfli"
)))]
const METHOD_DEFLATED: Option<zip_next::CompressionMethod> = None;

#[cfg(feature = "bzip2")]
const METHOD_BZIP2: Option<zip_next::CompressionMethod> = Some(zip_next::CompressionMethod::Bzip2);
#[cfg(not(feature = "bzip2"))]
const METHOD_BZIP2: Option<zip_next::CompressionMethod> = None;

#[cfg(feature = "zstd")]
const METHOD_ZSTD: Option<zip_next::CompressionMethod> = Some(zip_next::CompressionMethod::Zstd);
#[cfg(not(feature = "zstd"))]
const METHOD_ZSTD: Option<zip_next::CompressionMethod> = None;

fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        println!(
            "Usage: {} <source_directory> <destination_zipfile>",
            args[0]
        );
        return 1;
    }

    let src_dir = &*args[1];
    let dst_file = &*args[2];
    for &method in [METHOD_STORED, METHOD_DEFLATED, METHOD_BZIP2, METHOD_ZSTD].iter() {
        if method.is_none() {
            continue;
        }
        match doit(src_dir, dst_file, method.unwrap()) {
            Ok(_) => println!("done: {src_dir} written to {dst_file}"),
            Err(e) => println!("Error: {e:?}"),
        }
    }

    0
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip_next::CompressionMethod,
) -> zip_next::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip_next::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("adding file {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.start_file_from_path(name, options.clone())?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options.clone())?;
        }
    }
    zip.finish()?;
    Ok(())
}

fn doit(
    src_dir: &str,
    dst_file: &str,
    method: zip_next::CompressionMethod,
) -> zip_next::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(path).unwrap();

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

    Ok(())
}

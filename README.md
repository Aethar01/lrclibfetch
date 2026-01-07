# lrclibfetch

`lrclibfetch` is a CLI tool written in Rust that automatically fetches synced lyrics (`.lrc`) from [LRCLIB](https://lrclib.net/) for your music collection.

## Features

- **Automatic Metadata Extraction**: Reads artist, title, album, and duration from audio files using [lofty](https://github.com/Serial-0/lofty-rs).
- **LRCLIB Integration**: Fetches high-quality synced lyrics from the LRCLIB database.
- **Batch Processing**: Scans directories recursively for audio files.
- **Structure Preservation**: Mirrors the input directory structure in the output directory.

## Installation

### From Source

Ensure you have [Rust and Cargo](https://rustup.rs/) installed.

```bash
git clone https://github.com/yourusername/lrclibfetch.git
cd lrclibfetch
cargo build --release
```

## Usage

```bash
Usage: lrclibfetch [OPTIONS] --input <INPUT> --output-dir <OUTPUT_DIR>

Options:
  -b, --base-dir <BASE_DIR>      [default: .]
  -i, --input <INPUT>            
  -o, --output-dir <OUTPUT_DIR>  
  -h, --help                     Print help
  -V, --version                  Print version
```

### Example

If your music is organized as `~/Music/Artist/Album/Song.mp3`:

```bash
lrclibfetch --input ~/Music --output-dir ~/Lyrics --base-dir ~/Music
```

This will save the lyrics to `~/Lyrics/Artist/Album/Song.lrc`.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

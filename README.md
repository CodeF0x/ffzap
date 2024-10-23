ffzap ⚡ is a simple, fast command-line tool for processing media files with ffmpeg. As it's multithreaded and can run as
many tasks in parallel as your system can handle, it's perfect for converting, compressing, or editing audio and video
files quickly and efficiently.

Because it uses ffmpeg under the hood, it supports any media file processing that ffmpeg can handle.

### Installation

To install ffzap, clone the repository and build the project:

```bash
git clone https://github.com/CodeF0x/ffzap
cd ffzap
cargo build --release
```

Alternatively, you can install it from [crates.io](https://crates.io).

```bash
cargo install ffzap
```

### Usage

ffzap's usage is almost identical to ffmpeg, consider this simple example:

```bash
ffzap -i vids/test-1.webm -f "-c:v libx264 -b:v 1000k" -o transcoded.mp4
```

Mind that the ffmpeg processing options go into the `-f` argument (short for `--ffmpeg-options`), need to be passed
as a string and without the file name.

With a single file it doesn't really make sense to use ffzap, so consider this more advanced example:

```bash
ffzap -i "vids/**/*.{mp4,mkv}" -f "-c:v libx264 -b:v 1000k" -o transcoded/{{name}}.mp4 -t 4
```

This command takes all videos in `vids` and its subfolders ending in `.mp4` and `.mkv`, processes them using the
options provided by `-f` and saves them to a (new) directory called `transcoded`, keeping the original filename and
changing the file extension to `.mp4` while processing 4 files in parallel.

Notice that, when using glob patterns, `-i` (short for `--input-directory`) needs to be a string so that ffzap executes
the glob pattern and **not** your shell. Otherwise, the command will fail.

For more `-o` (short for `--output-directory`) options, run `ffzap --help`. For more ffmpeg options,
visit [ffmpeg's documentation](https://ffmpeg.org/ffmpeg.html).

### Requirements

- a working installation of [ffmpeg](https://ffmpeg.org/download.html)
- (just for installing / building) a working installation of
  the [Rust programming language](https://www.rust-lang.org/tools/install)

### License

You are free to:

- **Use**: You can use this software for any purpose.
- **Modify**: You can modify the software as you like.
- **Distribute**: You can distribute the original or lightly modified software only if you credit the original author (
  Tobias "CodeF0x" Oettl). Selling of the original or lightly modified versions is not allowed.
- **Sell or redistribute substantially modified versions**: If you make significant changes to this software, you may
  redistribute or sell the modified version without crediting the original author.

**Conditions**:

- No selling of this software in its original form or with minor modifications.
- Credit must be given to the original author (Tobias "CodeF0x" Oettl) if redistributing unmodified or minimally
  modified versions.

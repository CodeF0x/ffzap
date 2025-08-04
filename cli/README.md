ffzap ⚡ is a simple, fast command-line tool for processing media files with ffmpeg. As it's multithreaded and can run as
many tasks in parallel as your system can handle, it's perfect for converting, compressing, or editing audio and video
files quickly and efficiently.

Because it uses ffmpeg under the hood, it supports any media file processing that ffmpeg can handle.

1. [Installation](#installation)
2. [Usage](#usage)
   - [Practical examples](#more-practical-examples)
3. [Speed comparison to alternatives](#speed-comparison-to-commonly-used-alternatives-based-on-github-stars)
4. [Requirements](#requirements)
5. [All available options](#available-options)
6. [Migrating to 1.0.0](#migrating-to-100)
7. [License](#license)

### Installation

#### [Homebrew](https://brew.sh) (macOS / Linux)

```bash
brew tap CodeF0x/formulae
brew install ffzap
```

For macOS, both x86_64 and aarch64 builds are provided. For Linux, only x86_64.

#### [Winget](https://github.com/microsoft/winget-cli) (Windows 10 / 11)

```bash
winget install CodeF0x.ffzap
```

Only x86_64.

#### [Cargo](https://doc.rust-lang.org/cargo/) (Universal -- 32bit and 64bit)

(Install [Rust](https://www.rust-lang.org/tools/install) first if you haven't already)

```bash
cargo install ffzap
```

Every architecture Rust has compile-targets for.

#### Building from source

(Install [Rust](https://www.rust-lang.org/tools/install) first if you haven't already)

```bash
git clone https://github.com/CodeF0x/ffzap
cd ffzap
cargo build --release
```

Every architecture Rust has compile-targets for.

The ffzap executable will be under `target/release/`

#### Or [Download](https://github.com/CodeF0x/ffzap/releases/latest) a prebuilt binary

and add it to your path.

### Usage

ffzap's usage is almost identical to ffmpeg, consider this simple example:

```bash
ffzap -i vids/test-1.webm -f "-c:v libx264 -b:v 1000k" -o transcoded.mp4
```

Mind that the ffmpeg processing options go into the `-f` argument (short for `--ffmpeg-options`), need to be passed
as a string and without the file name.

With a single file it doesn't really make sense to use ffzap, so consider this more advanced example:

```bash
ffzap -i vids/**/*.{mp4,mkv} -f "-c:v libx264 -b:v 1000k" -o transcoded/{{name}}.mp4 -t 4
```

This command takes all videos in `vids` and its subfolders ending in `.mp4` and `.mkv`, processes them using the
options provided by `-f` and saves them to a (new) directory called `transcoded`, keeping the original filename and
changing the file extension to `.mp4` while processing 4 files in parallel.

For more info on the `-o` syntax, run `ffzap --help`. For more ffmpeg options,
visit [ffmpeg's documentation](https://ffmpeg.org/ffmpeg.html).

#### More practical examples:

<details>
<summary>Re-encode multiple videos to H265 and the audio to opus</summary>

```bash
ffzap --file-list files.txt -f "-c:v libx265 -preset medium -crf 23 -c:a libopus -b:a 128k" -o "Output/{{name}}.mp4" -t 2
```

Keypoints:

- use `--file-list` to pass a list of file names to process
- re-encode the video to H265 using `-c:v libx265`
  - `-preset medium` to balance out speed and file size
  - `-crf 23` to achieve good quality with reasonable file size
- re-encode the audio to opus using `-c:a libopus`
  - `-b:a 128k` sets the audio bitrate to 128k for a good trade-off between file size and audio quality
- `-t 2` runs two ffmpeg processes simultaneously to re-encode two files at once
  - adjust this number according to your system specs. Most system should be able to handle two instances comfortably

</details>

---

<details>
<summary>Convert PNG images to JPG</summary>

```bash
ffzap --file-list files.txt -f "-c:v mjpeg -q:v 2" -o "Output/{{name}}.jpg" -t 6
```

Keypoints:

- use `--file-list` to pass a list of file names to process
- convert the image to JPG using `-c:v mjpeg`
  - `-q:v 2` to set very high quality
- `-t 6` runs six processes in parallel, converting six files at once
  - adjust this number according to your system specs. Six shouldn't be too taxing on a modern CPU

</details>

---

<details>
<summary>Add a watermark to multiple videos</summary>

```bash
ffzap --file-list files.txt -f "-i watermark.png -filter_complex [1]format=rgba,lut=a=val*0.3[watermark];[0][watermark]overlay=(main_w-overlay_w)/2:(main_h-overlay_h)/2 -c:a copy" -o "{{name}}_watermark.mp4" -t 2
```

(Note that this command may not work in Windows Powershell as it requires a different escaping format)

Keypoints:

- use `--file-list` to pass a list of file names to process (these are the files the watermark gets added to)
- select to watermark file with `-i watermark.png` **inside** `-f`
- `-filter_complex` applies the watermark with 70% opacity to the center of each video
- `-c:a copy` copies the audio
- `-t 2` processes two files in parallel
  - adjust this number according to your system specs. Two should be good on most modern systems

</details>

---

<details>
<summary>Resize multiple videos</summary>

```bash
ffzap --file-list files.txt -f "-vf scale=1280:720 -c:a copy" -o "{{name}}_resized.mp4" -t 2
```

Keypoints:

- use `--file-list` to pass a list of file names to process
- `-vf scale=1280:720` sets the video resolution to HD
- `-c:a copy` copies the audio
- `-t 2` processes two files in parallel
  - adjust this number according to your system specs. Two should be good on most modern systems

</details>

---

<details>
<summary>Swap video containers</summary>

```bash
ffzap --file-list files.txt -o "{{name}}.mkv" -t 2
```

(It is assumed the source files have a container that's interchangeable with MKV)

Keypoints:

- use `--file-list` to pass a list of file names to process
- `-o "{{name}}.<desired file extension>` to swap all files to the desired container format (in this case MKV)
- No `-f` because it's not needed
- `-t 2` processes two files in parallel
  - adjust this number according to your system specs. Two should be good on most modern systems

</details>

### Speed comparison to commonly used alternatives (based on GitHub stars)

Preface: I want to point out that with this comparison, I am not saying the mentioned tools are bad or inferior to ffzap. They're achieving a great job and are used by many.
My personal issue with these tools is that I think they don't use system resources efficiently / don't allow the user to take control and this test aims to show that.

ffzap is not a magical tool that speeds up your batch encoding processes by simply increasing the amount of ffmpeg instances.
You need to be aware of what your system can and can't do and then leverage ffzap to take advantage of that.

---

Test system:

- Intel Core i7 13700K
- RTX 3090
- 32 GB RAM
- NVMe SSD

Test Videos (29):

- 1440p
- 60 FPS
- H264
- AAC / 192k

Re-Encoding settings:

- Video:
  - Codec: H265
  - Quality: CRF 24
  - Preset: medium
  - Resolution: 1080p
- Audio:
  - Copy

Used ffzap command:

```bash
ffzap -i Videos/**/*.mp4 -f "-c:v libx265 -crf 24 -preset medium -vf scale=1920:1080 -c:a copy" -o "Out/{{name}}.mp4" -t 2
```

In this example, we're comparing ffzap to other tools on how they utilize the system's CPU when re-encoding a video. An i7 13700K can run two parallel 1080p encodings comfortably when limited to 12 threads per ffmpeg instance, and we achieve that by using `-t 2` (`--threads 2`) in ffzap. This of course
slows down each individual process, but because we're running two jobs at once, it makes up for it.

All other mentioned tools (except FFBatch AV Converter) run only one encoding job at any given time, resulting in the following total encoding times:

| Software             | Version | Total Processing Time | Available Builds      | Gui | CLI |
| -------------------- | ------- | --------------------- | --------------------- | --- | --- |
| ffzap                | 1.0.0   | 00:32:16 (100%)       | Windows, Linux, macOS | No  | Yes |
| Handbrake            | 1.9.0   | 00:48:13 (149.5%)     | Windows, Linux, macOS | Yes | Yes |
| StaxRip              | 2.44.4  | 01:43:26 (320.5%)     | Windows               | Yes | Yes |
| FFBatch AV Converter | 3.2.1   | 00:30:44 (95.2%)      | Windows, Linux (Wine) | Yes | No  |

ffzap and FFBatch AV Converter are the clear winners here. Given the small difference in total encoding time between the two,
ffzap could just be as fast if not even faster if I would invest more time in finding the sweetspot between ffmpeg instances and threads per instance.
For the sake of simplicity for this example I didn't to that.

### Requirements

- a working installation of [ffmpeg](https://ffmpeg.org/download.html)
- (just for installing / building via Cargo) a working installation of
  the [Rust programming language](https://www.rust-lang.org/tools/install)

### Available options

<details>
  <summary>Click here to expand section</summary>

```bash
$ ffzap --help
⚡ A multithreaded CLI for digital media processing using ffmpeg. If ffmpeg can do it, ffzap can do it - as many files in parallel as your system can handle.

Usage: ffzap [OPTIONS] --output <OUTPUT>

Options:
-t, --thread-count <THREAD_COUNT>
        The amount of threads you want to utilize. most systems can handle 2. Go higher if you have a powerful computer. Default is 2. Can't be lower than 1

        [default: 2]

-f, --ffmpeg-options <FFMPEG_OPTIONS>
        Options you want to pass to ffmpeg. For the output file name, use --output

-i, --input <INPUT>...
        The files you want to process

    --file-list <FILE_LIST>
        Path to a file containing paths to process. One path per line

    --overwrite
        If ffmpeg should overwrite files if they already exist. Default is false

    --verbose
        If verbose logs should be shown while ffzap is running

    --delete
        Delete the source file after it was successfully processed. If the process fails, the file is kept

-o, --output <OUTPUT>
        Specify the output file pattern. Use placeholders to customize file paths:

        {{dir}}  - Entire specified file path, e.g. ./path/to/file.txt -> ?./path/to/

        {{name}} - Original file's name (without extension)

        {{ext}}  - Original file's extension

        Example: /destination/{{dir}}/{{name}}_transcoded.{{ext}}

        Outputs the file in /destination, mirroring the original structure and keeping both the file extension and name, while adding _transcoded to the name.

-h, --help
        Print help (see a summary with '-h')

-V, --version
        Print version
```

</details>

### Migrating to 1.0.0

In version `1.0.0`, the following changes were made:

- `--input-directory` has been deprecated and replaced by `--input`.
- `--input-file` has been deprecated and replaced by `--file-list`.

#### 1. Replacing `--input-directory` with `--input`:

Instead of using `--input-directory`, you now have to use `--input` to specify the files you want to process:

```bash
ffzap --input <files here> -f "<options here>" -o "<output pattern here>"
```

**Note:** The short form `-i` remains unaffected by this change.

#### 2. Replacing `--input-file` with `--file-list`:

Instead of `--input-file`, use `--file-list` to specify a file containing a list of files to process:

```bash
ffzap --file-list <path to list here> -f "<options here>" -o "<output pattern here>"
```

---

For further details and motivation behind these changes, refer to [issue 16](https://github.com/CodeF0x/ffzap/issues/16).

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

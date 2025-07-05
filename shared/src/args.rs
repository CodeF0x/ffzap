use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct CmdArgs {
    /// The amount of threads you want to utilize. most systems can handle 2. Go higher if you have a powerful computer. Default is 2. Can't be lower than 1
    #[arg(short, long, default_value_t = 2, value_parser = clap::value_parser!(u16).range(1..))]
    pub thread_count: u16,

    /// Options you want to pass to ffmpeg. For the output file name, use --output
    #[arg(short, long, allow_hyphen_values = true)]
    pub ffmpeg_options: Option<String>,

    /// The files you want to process.
    #[arg(short, long, num_args = 1.., required_unless_present = "file_list", conflicts_with = "file_list")]
    pub input: Option<Vec<String>>,

    /// Path to a file containing paths to process. One path per line
    #[arg(long, required_unless_present = "input", conflicts_with = "input")]
    pub file_list: Option<String>,

    /// If ffmpeg should overwrite files if they already exist. Default is false
    #[arg(long, default_value_t = false)]
    pub overwrite: bool,

    /// If verbose logs should be shown while ffzap is running
    #[arg(long, default_value_t = false)]
    pub verbose: bool,

    /// Delete the source file after it was successfully processed. If the process fails, the file is kept.
    #[arg(long, default_value_t = false)]
    pub delete: bool,

    /// Displays the current eta in the progressbar
    #[arg(long, default_value_t = false)]
    pub eta: bool,

    /// Specify the output file pattern. Use placeholders to customize file paths:
    ///
    /// {{dir}}  - Entire specified file path, e.g. ./path/to/file.txt -> ?./path/to/
    ///
    /// {{name}} - Original file's name (without extension)
    ///
    /// {{ext}}  - Original file's extension
    ///
    /// Example: /destination/{{dir}}/{{name}}_transcoded.{{ext}}
    ///
    /// Outputs the file in /destination, mirroring the original structure and keeping both the file extension and name, while adding _transcoded to the name.
    #[arg(short, long)]
    pub output: String,
}
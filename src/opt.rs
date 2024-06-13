use std::{fmt::Display, str::FromStr};

use clap::Parser;

//CLI 命令
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}
#[derive(Debug, Parser)]
pub enum SubCommand {
    //命令行
    #[command(name = "csv", about = "Show CSV,or convert CSV to other format")]
    Csv(CsvOpts), //csv功能的cli选项
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    //定义了Csv功能的选项
    #[arg(short,long,value_parser = verify_input_file)]
    pub input: String, //输入的文件路径
    #[arg(short, long, default_value = "output")]
    pub output: String, //输出的文件路径默认为output.json
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char, //文件的分割符号默认为,
    #[arg(long, default_value_t = true)]
    pub header: bool, //文档是否有header默认有
    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
}
#[derive(Debug, Parser, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}
//检查输入的文件路径的合法性
fn verify_input_file(file_name: &str) -> Result<String, String> {
    if std::path::Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("Input file not exist.".into())
    }
}

//转换命令行参数为对应的格式
fn parse_format(format: &str) -> Result<OutputFormat, String> {
    format.parse()
}
impl From<OutputFormat> for &str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Toml => "toml",
            OutputFormat::Yaml => "yaml",
        }
    }
}
impl FromStr for OutputFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => Err("Invalid format".into()),
        }
    }
}
impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self)) //写入转换为的&str
    }
}

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
    #[arg(short, long, default_value = "output.json")]
    pub output: String, //输出的文件路径默认为output.json
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char, //文件的分割符号默认为,
    #[arg(long, default_value_t = true)]
    pub header: bool, //文档是否有header默认有
}
//检查输入的文件路径的合法性
fn verify_input_file(file_name: &str) -> Result<String, String> {
    if std::path::Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("Input file not exist.".into())
    }
}

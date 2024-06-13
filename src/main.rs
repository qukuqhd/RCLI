// csv 功能： rcli csv -i input.csv -o output.json --header -d ","

use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

//CLI 命令
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}
#[derive(Debug, Parser)]
enum SubCommand {
    //命令行
    #[command(name = "csv", about = "Show CSV,or convert CSV to other format")]
    Csv(CsvOpts), //csv功能的cli选项
}
#[derive(Debug, Parser)]
struct CsvOpts {
    //定义了Csv功能的选项
    #[arg(short,long,value_parser = verify_input_file)]
    input: String, //输入的文件路径
    #[arg(short, long, default_value = "output.json")]
    output: String, //输出的文件路径默认为output.json
    #[arg(short, long, default_value_t = ',')]
    delimiter: char, //文件的分割符号默认为,
    #[arg(long, default_value_t = true)]
    header: bool, //文档是否有header默认有
}

fn main() {
    let opts = Opts::parse(); //解析传入的命令行，根据我们定义的数据结构来进行参数合法性的判断
                              //匹配不同可能的命令行选项
    match opts.cmd {
        SubCommand::Csv(option) => {
            let mut reader = Reader::from_path(option.input).unwrap();
            //反序列化得到对象列表
            let records = reader
                .deserialize()
                .map(|record| record.unwrap())
                .collect::<Vec<Player>>();
            println!("{:?}", records);
        }
    }
}
//检查输入的文件路径的合法性
fn verify_input_file(file_name: &str) -> Result<String, String> {
    if std::path::Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("Input file not exist.".into())
    }
}

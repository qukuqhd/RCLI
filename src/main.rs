// csv 功能： rcli csv -i input.csv -o output.json --header -d ","

use clap::Parser;
use csv::Reader;
use rcli::opt::*;
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
fn main() {
    let opts = Opts::parse(); //解析传入的命令行，根据我们定义的数据结构来进行参数合法性的判断
                              //匹配不同可能的命令行选项
    match opts.cmd {
        SubCommand::Csv(option) => {
            let mut reader = Reader::from_path(option.input).unwrap();
            //反序列化得到对象列表
            let ret = reader
                .deserialize()
                .map(|record| record.unwrap())
                .collect::<Vec<Player>>();
            let json = serde_json::to_string_pretty(&ret).unwrap();
            std::fs::write(option.output, json).unwrap();
        }
    }
}

// csv 功能： rcli csv -i input.csv -o output.json --header -d ","

use clap::Parser;
use csv::Reader;
use rcli::opt::*;

fn main() {
    let opts = Opts::parse(); //解析传入的命令行，根据我们定义的数据结构来进行参数合法性的判断
                              //匹配不同可能的命令行选项
    match opts.cmd {
        SubCommand::Csv(option) => {
            let mut reader = Reader::from_path(option.input).unwrap();
            //反序列化得到对象列表
            let headers = reader.headers().unwrap().clone();
            let ret: Vec<serde_json::Value> = reader
                .records()
                .map(|record| {
                    headers
                        .iter()
                        .zip(record.unwrap().iter())
                        .collect::<serde_json::Value>()
                })
                .collect();

            let content = match option.format {
                OutputFormat::Json => serde_json::to_string(&ret).unwrap(),
                OutputFormat::Toml => toml::to_string(&ret).unwrap(),
                OutputFormat::Yaml => serde_yaml::to_string(&ret).unwrap(),
            };
            std::fs::write(format!("{}.{}", option.output, option.format), content).unwrap();
        }
    }
}

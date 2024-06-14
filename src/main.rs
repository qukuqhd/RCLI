// csv 功能： rcli csv -i input.csv -o output.json --header -d ","

use clap::Parser;
use rcli::{
    opt::*,
    process::{csv_convert::process_csv, gen_pass::process_genpass},
};
use zxcvbn::zxcvbn;
fn main() {
    let opts = Opts::parse(); //解析传入的命令行，根据我们定义的数据结构来进行参数合法性的判断
                              //匹配不同可能的命令行选项
    match opts.cmd {
        SubCommand::Csv(option) => {
            process_csv(option.input, option.format, option.output);
        }
        SubCommand::GenPass(option) => {
            let pwd = process_genpass(
                option.length,
                option.uppercase,
                option.lowwerase,
                option.number,
                option.number,
            )
            .unwrap();
            println!("passwd is {}", pwd);
            eprintln!("密码强度为{:?}", zxcvbn(&pwd, &[]).unwrap().score());
        }
    }
}

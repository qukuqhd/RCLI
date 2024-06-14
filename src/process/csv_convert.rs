use csv::Reader;

use crate::opt::OutputFormat;

pub fn process_csv(input: String, format: OutputFormat, output: String) {
    let mut reader = Reader::from_path(input).unwrap();
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

    let content = match format {
        OutputFormat::Json => serde_json::to_string(&ret).unwrap(),
        OutputFormat::Toml => toml::to_string(&ret).unwrap(),
        OutputFormat::Yaml => serde_yaml::to_string(&ret).unwrap(),
    };
    std::fs::write(format!("{}.{}", output, format), content).unwrap();
}

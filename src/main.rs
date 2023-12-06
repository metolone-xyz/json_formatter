use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use serde_json::Value;
use regex::Regex;

fn main() -> io::Result<()> {
    // JSONファイルのパス
    let file_path = "/Users/metolone/Documents/video_data.json";

    // ファイルを読み込む
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    // JSONデータをパースする
    let mut json: Vec<Value> = serde_json::from_str(&data)?;

    // URLを識別するための正規表現
    let url_regex = Regex::new(r"(http://www\.|https://www\.|http://|https://|www\.)\S+").unwrap();

    // JSON配列内の各アイテムを処理する
    for item in json.iter_mut() {
        if let Some(output) = item["output"].as_str() {
            // 全角スペースを半角スペースに置き換える
            let replaced_output = output.replace("　", " ");
            // URLを削除する
            let cleaned_output = url_regex.replace_all(&replaced_output, "");
            item["output"] = Value::from(cleaned_output.to_string());
        }
    }

    // 変更をファイルに保存する
    let mut file = OpenOptions::new().write(true).truncate(true).open(file_path)?;
    file.write_all(serde_json::to_string_pretty(&json)?.as_bytes())?;

    Ok(())
}

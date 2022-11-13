extern crate regex;
use regex::Regex;

use std::fs::File;
use std::fs;
use std::io::{Error, Read, Write};
use std::path::Path;
use std::env;
use colored::Colorize;

// ファイル書き込み
fn file_create(name: &String, contents: &String) -> Result<(), Error> {
    let mut f = File::create(name.trim())?;
    f.write_all(contents.as_bytes())?;

    Ok(())
}

// ファイル読み込み
fn get_file_contents(name: &String) -> Result<String, Error> {
    let mut f = File::open(name.trim())?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}

// 指定したファイルを crlf -> lf へ変換
fn replace_file_indention_code(name: &String) -> Result<(), Error> {
    let mut contents = get_file_contents(&name)?;
    contents = contents.replace("\r\n","\n");
    file_create(&name, &contents)?;

    Ok(())
}

// 指定したフォルダ以下のファイルの改行コードを変更する
fn replace_files<P: AsRef<Path>>(path: P, re: &Regex) -> Result<(), Error> {
    if let Ok(dirs) = fs::read_dir(path){
        for entry in dirs.into_iter() {
            if let Ok(entry) = entry{
                let path = entry.path();
                let path_str = path.to_string_lossy().into_owned();

                if entry.file_type()?.is_dir(){
                    replace_files(path, re)?;
                }
                else if re.is_match(&path_str) && entry.file_type()?.is_file()  {
                    replace_file_indention_code(&path_str)?;
                    println!("{} : {}", format!("fix").yellow(), path_str);
                }
            }
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let search_root_foldter = &args[1];
    let search_file_match_pattern = &args[2];
    
    if let Ok(re) = Regex::new(search_file_match_pattern){
        match replace_files(&search_root_foldter, &re){
            Ok(()) => {
                println!("{}", format!("Finish").cyan().bold());
            }
            Err(e) => {
                println!("{} : {}", format!("Error!").red().bold(), e);
            }
        }
    }
}

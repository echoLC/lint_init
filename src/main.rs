use ansi_term::Colour::{Red, Green};
use clap::Parser;
use std::fs;
use std::io;
use std::env::current_dir;
use std::path::{PathBuf};
use relative_path::RelativePath;

#[derive(Parser, Debug)]
#[clap(name="lint-init", author="echoLC", version = "0.1.0", about = "Init lint config for a project.", long_about=None)]
struct Cli {
    #[clap(short, long)]
    template: String,

    #[clap(short, long, default_value = ".")]
    dir: String,
}

const ESLINT_FILE_NAME: &str = "/.eslintrc.json";

struct FileInfo {
    template_url: String,
    target_url: String
}

fn main() {
    let args = Cli::parse();
    let template = args.template;
    let dir = args.dir;

    let file_info: FileInfo = match template.as_str() {
        "typescript" => FileInfo{template_url: String::from("/typescript.json"), target_url: String::from(ESLINT_FILE_NAME) },
        "reactTs" => FileInfo{template_url: String::from("/react.json"), target_url: String::from(ESLINT_FILE_NAME) },
        "prettier" => FileInfo{template_url: String::from("/prettier.js"), target_url: String::from("/.prettierrc.js") },
        "pureJs" => FileInfo{template_url: String::from("/pure-js.json"), target_url: String::from(ESLINT_FILE_NAME) },
        _ => panic!("unknown template type: {}", Red.paint(template))
    };

    let current_dir_s = if let Ok(path) = current_dir() {
        path
    } else {
        panic!("Unable get current dir");
    };

    let current_dir_s = match current_dir_s.to_str() {
        Some(path_str) => path_str,
        None => panic!("Unable get current dir")
    };

    println!("template url: {:?}",  String::from(current_dir_s) + "/src/templates" + &file_info.template_url);

    let template_url = String::from(current_dir_s) + "/src/templates" + &file_info.template_url;
    let target_url = file_info.target_url;
    

    let content = read_template_content(template_url).expect("Unable to read file");

    println!("template content is: {}", Green.paint(&content));

    let target_dir = PathBuf::from(&dir);
    let target_dir = fs::canonicalize(&target_dir);

    match &target_dir {
        Ok(path) => {
            print!("write file content");

            let target_url_prefix = match path.to_str() {
                Some(path_str) => path_str,
                None => panic!("Unable get current target path")
            };

            write_content(&content, String::from(target_url_prefix) + &target_url);
        },
        Err(_err) => {
            let target_path =  get_target_dir(&dir);

            fs::create_dir(&target_path).expect("Unable to create dir");

            let target_url_prefix = match target_path.to_str() {
                Some(path_str) => path_str,
                None => panic!("Unable get current dir")
            };

            write_content(&content, String::from(target_url_prefix) + &target_url);
        }
    };
}

fn read_template_content (path: String) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn write_content(content: &str, target_path: String) {
    let res = fs::write(&target_path,content);
            
    match res {
        Ok(_) => println!("write {} config successfully", &target_path),
        Err(err) => panic!("write file failed: {:?}", err)
    }
}

fn get_target_dir (dir: &str) -> PathBuf {
    let target_dir_path = current_dir().unwrap();
    let current_dir_s = match target_dir_path.to_str() {
        Some(path_str) => path_str,
        None => panic!("Unable get current dir")
    };
    let target_dir_path = current_dir_s.to_string() + "/" + dir;
    let target_dir_path = RelativePath::new(&target_dir_path).normalize();

    println!("target dir is : {:?}", &target_dir_path);
    
    target_dir_path.to_path("/")
}

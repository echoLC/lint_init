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
    let dir = args.dir;

    let file_info: FileInfo = get_template(args.template);

    let current_dir_path = if let Ok(path) = current_dir() {
        path
    } else {
        panic!("Unable get current dir");
    };

    let template_url = get_str_from_pathbuf(current_dir_path) + "/src/templates" + &file_info.template_url;
    let target_url = file_info.target_url;
    

    let content = read_template_content(template_url).expect("Unable to read file");

    println!("template content is: {}", Green.paint(&content));

    let target_dir = PathBuf::from(&dir);
    let target_dir = fs::canonicalize(&target_dir);

    match &target_dir {
        Ok(path) => {
            print!("write file content");

            write_content(&content, get_str_from_pathbuf(path.to_path_buf()) + &target_url);
        },
        Err(_err) => {
            let target_path =  get_target_dir(&dir);

            fs::create_dir(&target_path).expect("Unable to create dir");

            write_content(&content, get_str_from_pathbuf(target_path) + &target_url);
        }
    };
}

fn get_template (template: String) -> FileInfo {
    match template.as_str() {
        "typescript" => FileInfo{template_url: String::from("/typescript.json"), target_url: String::from(ESLINT_FILE_NAME) },
        "reactTs" => FileInfo{template_url: String::from("/react.json"), target_url: String::from(ESLINT_FILE_NAME) },
        "prettier" => FileInfo{template_url: String::from("/prettier.js"), target_url: String::from("/.prettierrc.js") },
        "pureJs" => FileInfo{template_url: String::from("/pure-js.json"), target_url: String::from(ESLINT_FILE_NAME) },
        _ => panic!("unknown template type: {}", Red.paint(template))
    }
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
    let current_dir_s = get_str_from_pathbuf(current_dir().unwrap());
    let target_dir_path = current_dir_s + "/" + dir;
    let target_dir_path = RelativePath::new(&target_dir_path).normalize();

    println!("target dir is : {:?}", &target_dir_path);
    
    target_dir_path.to_path("/")
}

fn get_str_from_pathbuf (path: PathBuf) -> String {
    match path.to_str() {
        Some(path_str) => path_str.to_string(),
        None => panic!("Unable get string from pathbuf")
    }
}

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Debug)]
struct FileBlock {
    filename: String,
    raw_name: String,
    imports: Vec<String>,
    content: String,
}

fn main() {
    let full_test_file = "./src/test-files/flutter/widget.dart";
    let nested_test_file = "./src/test-files/flutter/nested_many_brackets.dart";
    let file = File::open(full_test_file).unwrap();
    let buf_reader = BufReader::new(file);
    let mut raw_lines: Vec<String> = Vec::new();
    for line in buf_reader.lines() {
        if let Ok(text) = line {
            raw_lines.push(text);
        }
    }

    let mut imports_list = Vec::new();

    // Extracting main files
    let extracted_main_file_contents: Vec<String> = raw_lines
        .iter()
        .map(|text| {
            if !text.starts_with("//") && !text.starts_with("import") {
                return text.clone();
            } else if text.starts_with("import") {
                imports_list.push(text.clone());
                return String::from("\n");
            } else {
                return String::from("\n");
            }
        })
        .filter(|text| !text.is_empty())
        .collect();

    // open bracket counter, if open bracket counter value is 0 then the element contains the
    // matching  close bracket. If not found matching bracket yet, should concat current string

    let mut file_content = String::new();
    let mut open_bracket_count = 0;
    let mut file_contents = Vec::new();

    extracted_main_file_contents.iter().for_each(|content| {
        if content.contains("{") && content.contains("}") {
            if open_bracket_count == 0 {
                let content_block = format!("{}", file_content.clone() + content);
                file_contents.push(content_block);
                file_content = String::new();
            }
        } else if content.contains("{") {
            file_content = format!("{}", file_content.clone() + content);
            open_bracket_count += 1;
        } else if content.contains("}") {
            open_bracket_count -= 1;
            if open_bracket_count == 0 {
                let content_block = format!("{}", file_content.clone() + "}");
                file_contents.push(content_block);
                file_content = String::new();
            } else {
                file_content = format!("{}", file_content.clone() + content);
            }
        } else {
            file_content = format!("{}", file_content.clone() + content);
        }
    });

    let mut imports: HashMap<String, String> = HashMap::new();
    // let mut file_names: Vec<String> = Vec::new();

    let mut raw_names: Vec<String> = file_contents
        .iter()
        .map(|content| {
            let raw_name = get_raw_name(content);
            let file_name = get_file_name(content);
            let path = env::current_dir();
            let last = path.unwrap().display().to_string().split("/").last();
            let curr = last.unwrap();
            imports.insert(
                raw_name.clone(),
                format!("package:{}/{}.dart", curr_dir, file_name.unwrap(),),
            );
            raw_name
        })
        .collect();

    println!("{:?}", raw_names);
    println!("{:?}", imports);

    let mut stateful_content = String::new();
    let mut stateful_count = 0;
    let mut stateful_filename = String::new();
    let list_of_files: Vec<Option<FileBlock>> = file_contents
        .iter()
        .map(|file| {
            let content = file.to_string();
            let filename = get_file_name(&content);
            let raw_name = get_raw_name(&content);
            let is_stateful = content.contains(&"StatefulWidget".to_string());
            if !is_stateful && stateful_count == 0 {
                stateful_content = String::new();
                return Some(FileBlock {
                    filename: filename.unwrap(),
                    raw_name,
                    content,
                    imports: imports_list.clone(),
                });
            } else {
                if stateful_count == 1 {
                    stateful_content = format!("{}", stateful_content.clone() + &content);
                    stateful_count = 0;
                    let file = Some(FileBlock {
                        filename: stateful_filename.clone(),
                        raw_name,
                        content: stateful_content.clone(),
                        imports: imports_list.clone(),
                    });
                    stateful_filename = String::new();
                    return file;
                }
                stateful_count = 1;
                stateful_content = content.clone();
                stateful_filename = filename.unwrap();
                return None;
            }
        })
        .collect();

    println!("{:?}", list_of_files);
    // export_files(list_of_files);

    // TODO:
    // Determine which files belong to a file to determine the imports
    // For now just copy the default import in the uncleaved file then add the necessary imports
    // for each file
    // You can get full path import using the directory/project name like : package:<dir_name>/ignore/my_custom_button.dart
}

fn export_files(list: Vec<Option<FileBlock>>) {
    list.iter()
        .filter(|content| content.is_some())
        .for_each(|file| {
            let dir_path = "./src/outputs/".to_string();
            let file_name = file.as_ref().unwrap().filename.clone();
            let contents = file.as_ref().unwrap().content.clone();
            let full_path = format!("{}", dir_path + &file_name + ".dart");
            let f = File::create_new(full_path);
            let imports = file.as_ref().unwrap().imports.join("\n");
            let full_file_content = format!("{}", imports + &contents);
            if let Ok(mut file) = f {
                let _ = file.write_all(full_file_content.as_bytes());
            }
        })
}

fn get_raw_name(file_text: &String) -> String {
    let texts: Vec<String> = file_text
        .split_whitespace()
        .map(|t| String::from(t))
        .collect();
    if texts.contains(&"StatelessWidget".to_string())
        || texts.contains(&"StatefulWidget".to_string())
    {
        let file_name = texts.get(1);
        file_name.unwrap().to_string()
    } else if texts.contains(&"main()".to_string()) {
        "main".to_string()
    } else {
        let function_texts: Vec<&str> = texts.get(1).unwrap().split("(").collect();
        let file_name = function_texts.get(0);
        file_name.unwrap().to_string()
    }
}

fn get_file_name(file_text: &String) -> Option<String> {
    let texts: Vec<String> = file_text
        .split_whitespace()
        .map(|t| String::from(t))
        .collect();
    if texts.contains(&"StatelessWidget".to_string())
        || texts.contains(&"StatefulWidget".to_string())
    {
        let file_name = texts.get(1);
        return match file_name {
            Some(text) => {
                return format_filename_snakecase(Some(text.to_string()));
            }
            None => None,
        };
    } else if texts.contains(&"main()".to_string()) {
        Some("main".to_string())
    } else {
        let function_texts: Vec<&str> = texts.get(1).unwrap().split("(").collect();
        let file_name = function_texts.get(0);
        /* if let Some(name) = file_name {
            return format_filename(Some(name.to_string()));
        } */

        // None
        if let Some(name) = file_name {
            return Some(name.to_string());
        } else {
            None
        }
    }
}

fn format_filename_snakecase(file_name: Option<String>) -> Option<String> {
    return match file_name {
        Some(text) => {
            let mut texts: Vec<String> = Vec::new();
            let mut word = String::new();
            let chars = text.chars();
            for char in chars {
                if char.is_uppercase() {
                    if !word.is_empty() {
                        texts.push(word);
                    }
                    word = String::new();
                    word.push(char);
                } else {
                    word.push(char);
                }
            }
            texts.push(word);
            let lower_case_texts = texts
                .iter()
                .map(|text| text.to_lowercase())
                .collect::<Vec<String>>();

            return Some(lower_case_texts.join("_").to_string());
        }
        None => None,
    };
}

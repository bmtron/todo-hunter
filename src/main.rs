use clap::Parser;
use git2::{Index, Repository};
use std::fs;
use std::io::{BufRead, BufReader};

mod errors;
use crate::errors::InvalidPathError;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./")]
    path: String,
    #[arg(short, long)]
    tag: Option<String>,
}

// TODO_BM: move these structs to their own file
// as well as their implementations /td
struct FileMap {
    ignored_paths: Vec<String>,
    included_paths: Vec<String>,
}

struct TodoMessage {
    line_number: u16,
    message: String,
    file_name: String,
}

impl TodoMessage {
    pub fn new() -> Self {
        TodoMessage {
            line_number: 0,
            message: String::new(),
            file_name: String::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let target_path = &args.path;
    if let Some(last_char) = target_path.chars().last() {
        if last_char != '/' {
            return Err(Box::new(InvalidPathError::new(
                "Invalid path. Must append '/' to path argument.",
            )));
        }
    }

    let mut todo_token = String::from("TODO");
    if let Some(tag) = args.tag {
        let tag = tag.as_str();
        todo_token.push_str("_");
        todo_token.push_str(tag);
    }

    let repo = match Repository::open(target_path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let index = repo.index()?;
    let file_map = build_file_map(&index, target_path)?;

    hunt_for_todos(&file_map, &todo_token, &target_path)?;
    Ok(())
}

fn hunt_for_todos(
    map: &FileMap,
    todo_token: &str,
    main_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut message_vec: Vec<TodoMessage> = Vec::new();

    /* TODO: handle this part better.
    I don't like relying on a tag to end the todo. /td */
    for path in map.included_paths.iter() {
        let mut temp_owned_path = main_path.to_owned();
        temp_owned_path.push_str(path);
        let working_file = std::fs::File::open(temp_owned_path)?;

        let reader = BufReader::new(working_file);
        let mut count = 1;
        let mut line_start = 0;
        let mut msg: String = String::new();
        let mut in_comment = false;
        let mut in_block_comment = false;
        let mut in_todo_msg = false;

        for line in reader.lines() {
            let line_result = match &line {
                Ok(_l) => true, 
                Err(_e) => false
            };
            
            if !line_result {
                continue;
            }

            let unwrp_line: String = line?;

            if unwrp_line.trim_start().starts_with("//")
                || unwrp_line.trim_start().starts_with("/*")
            {
                in_comment = true;
                if unwrp_line.trim_start().starts_with("/*") {
                    in_block_comment = true;
                }
            } else {
                in_comment = false;
            }

            if unwrp_line.trim_end().ends_with("*/") {
                in_block_comment = false;
            }

            if in_comment || in_block_comment {
                if unwrp_line.contains(todo_token) {
                    in_todo_msg = true;
                    line_start = count;
                    msg.push_str(unwrp_line.trim_start());
                    msg.push_str("\n");
                    // TODO: set this formatting up properly /td
                    // this is exactly 11 spaces.
                    // for formatting output on wrapped lines
                    msg.push_str("           ");
                } else if in_todo_msg {
                    msg.push_str(unwrp_line.trim_start());
                    msg.push_str("\n");
                    msg.push_str("           ");
                }
            }

            // end of todo, leaves room for other comments inside of block
            // TODO: check if this works /td
            if in_todo_msg {
                if unwrp_line.trim_end().ends_with("/td") || (!in_comment && !in_block_comment) {
                    let mut todomsg: TodoMessage = TodoMessage::new();
                    todomsg.line_number = line_start;
                    todomsg.file_name = path.to_owned();
                    todomsg.message = msg.trim_end().to_owned().clone();
                    message_vec.push(todomsg);
                    msg = String::new();
                    in_todo_msg = false;
                }
            }

            count = count + 1;
        }
    }
    // TODO handle multiple line formatting in a more professional way /td
    for item in message_vec.iter() {
        println!("{}:{}:{}", item.file_name, item.line_number, item.message);
    }
    Ok(())
}

fn build_file_map(index: &Index, path: &str) -> Result<FileMap, Box<dyn std::error::Error>> {
    let mut map: FileMap = FileMap {
        ignored_paths: Vec::new(),
        included_paths: Vec::new(),
    };
    let ignore_file = ".gitignore";
    for entry in index.iter() {
        let entry_path = std::str::from_utf8(&entry.path)?;
        if entry_path == ignore_file {
            let mut owned_path: String = path.to_owned();
            owned_path.push_str(ignore_file);
            build_ignore_map(&mut map.ignored_paths, owned_path.as_str())?;
        } else {
            map.included_paths.push(entry_path.to_owned());
        }
    }

    Ok(map)
}

fn build_ignore_map(
    ignored_paths: &mut Vec<String>,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let gitignore = std::fs::File::open(path)?;
    let reader = BufReader::new(gitignore);

    for line in reader.lines() {
        let lineclone: String = line?;
        ignored_paths.push(lineclone);
    }

    Ok(())
}

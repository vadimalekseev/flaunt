use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::Difficult::{Easy, Hard, Medium};

fn main() {
    let args = parse_args();

    let to_parse = vec!["hard", "medium", "easy"];

    let folder_path = Path::new(&args.folder);

    for difficult_name in to_parse {
        let folder = folder_path.join(difficult_name);
        let path = folder.to_str().unwrap();

        if !folder.exists() {
            println!("folder {} does not exists", path);
            continue;
        }
        if !folder.is_dir() {
            println!("can't read folder {}, it is a file", path);
            continue;
        }

        for solving_file in folder.read_dir().unwrap().into_iter().filter_map(|x| x.ok()) {
            let content = fs::read_to_string(solving_file.path()).unwrap();

            let path = solving_file.path();
            let extension = path.extension().unwrap();

            let solving_info = parse_solving(difficult_name, extension.to_str().unwrap().to_string(), content);
            println!("{}, {}", solving_info.comment.unwrap_or_else(|| "empty comment".to_string()), solving_info.id.to_string());
        }
    }
}

struct Args {
    folder: String,
}

fn parse_args() -> Args {
    let args = std::env::args().collect::<Vec<_>>();

    let mut folder = ".".to_string();
    let mut next_folder = false;

    for arg in args.get(1..).unwrap() {
        if arg == "--folder" {
            next_folder = true;
            continue;
        }
        if next_folder {
            folder = arg.clone();
            next_folder = false;
            continue;
        }

        panic!("exta arguments. Usage: flaunt --folder /path/to/solving");
    }

    return Args {
        folder,
    };
}

enum Difficult {
    Hard,
    Medium,
    Easy,
}

impl FromStr for Difficult {
    type Err = ();

    fn from_str(input: &str) -> Result<Difficult, Self::Err> {
        match input {
            "hard" => Ok(Hard),
            "medium" => Ok(Medium),
            "easy" => Ok(Easy),
            _ => Err(()),
        }
    }
}


struct Solving {
    id: String,
    difficult: Difficult,
    comment: Option<String>,
}

fn parse_solving(difficult: &str, filename: String, solving: String) -> Solving {
    let first_comment = solving
        .lines()
        .take_while(|x| x.starts_with("//") || x.starts_with("--") || x.starts_with("##"))
        .next();

    let commentless = match first_comment {
        Some(x) => Some(x.get("//".len()..).unwrap().trim().to_string()),
        None => None,
    };


    return Solving {
        id: filename,
        difficult: Difficult::from_str(difficult).unwrap(),
        comment: commentless,
    };
}

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::path::Path;
use std::str::FromStr;

struct Problem {
    id: String,
    difficult: Difficult,
    solvings: Vec<Solving>,
}

struct Solving {
    language: String,
    path: String,
    comment: Option<String>,
}

fn main() {
    let args = parse_args();

    let difficults = vec!["hard", "medium", "easy"];

    let folder_path = Path::new(&args.folder);

    let mut problems: HashMap<String, Problem> = HashMap::new();
    for difficult_name in difficults {
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

        for solving_file in folder
            .read_dir()
            .unwrap()
            .into_iter()
            .filter_map(|x| x.ok())
        {
            let content = fs::read_to_string(solving_file.path()).unwrap();

            let path = solving_file.path();
            let leetcode_id = path.file_stem().unwrap().to_str().unwrap().to_string();

            let stripped_path = path.strip_prefix(folder_path).unwrap();
            let solving_info = parse_solving(stripped_path, content);

            match problems.entry(leetcode_id.to_owned()) {
                Entry::Occupied(mut e) => {
                    e.get_mut().solvings.push(solving_info);
                }
                Entry::Vacant(e) => {
                    e.insert(Problem {
                        id: leetcode_id,
                        difficult: Difficult::from_str(difficult_name).unwrap(),
                        solvings: vec![solving_info],
                    });
                }
            }
        }
    }

    println!("{}", generate(problems))
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

    return Args { folder };
}

#[derive(PartialEq, Eq, Hash)]
enum Difficult {
    Hard,
    Medium,
    Easy,
}

impl FromStr for Difficult {
    type Err = ();

    fn from_str(input: &str) -> Result<Difficult, Self::Err> {
        match input {
            "hard" => Ok(Difficult::Hard),
            "medium" => Ok(Difficult::Medium),
            "easy" => Ok(Difficult::Easy),
            _ => Err(()),
        }
    }
}

impl Difficult {
    fn to_str(&self) -> String {
        return match self {
            Difficult::Hard => "Hard".to_string(),
            Difficult::Medium => "Medium".to_string(),
            Difficult::Easy => "Easy".to_string(),
        };
    }
}

// todo: allow user mappings
fn map_lang(s: &str) -> &str {
    match s {
        "rs" => "Rust",
        "cs" => "C#",
        "go" => "Go",
        "py" => "Python",
        "js" => "JS",
        "ts" => "TS",
        "rb" => "Ruby",
        "cpp" => "C++",
        "c" => "C",
        "sql" => "SQL",
        _ => s,
    }
}

fn parse_solving(path: &Path, solving: String) -> Solving {
    let extension = path.extension().unwrap();

    let first_comment = solving
        .lines()
        .take_while(|x| x.starts_with("//") || x.starts_with("--") || x.starts_with("##"))
        .next();

    let comment = match first_comment {
        Some(x) => Some(x.get("//".len()..).unwrap().trim().to_string()),
        None => None,
    };

    return Solving {
        comment,
        path: path.to_str().unwrap().to_string(),
        language: map_lang(extension.to_str().unwrap()).to_string(),
    };
}

fn pascal_case(s: &String) -> String {
    return s
        .to_owned()
        .split("-")
        .map(|s| format!("{}{}", (&s[..1].to_string()).to_uppercase(), &s[1..]))
        .collect::<Vec<String>>()
        .join(" ");
}

fn leetcode_problem_url(problem: &String) -> String {
    format!("https://leetcode.com/problems/{}", problem)
}

fn generate_table_body(problems: Vec<&Problem>) -> String {
    let mut idx = 0;
    return problems
        .iter()
        .map(|problem| {
            idx += 1;
            let solvings = problem
                .solvings
                .iter()
                .map(|x| {
                    let comment = match x.comment.to_owned() {
                        Some(v) => format!("({v})"),
                        None => String::from(""),
                    };
                    format!("({})[{}]{}", x.language, x.path, comment)
                })
                .collect::<Vec<String>>()
                .join(", ");

            format!(
                "|{}|[{}]({})|{}|{}|",
                idx,
                pascal_case(&problem.id),
                leetcode_problem_url(&problem.id),
                problem.difficult.to_str(),
                solvings
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
}

fn generate_solvings_table(summary: &str, problems: Vec<&Problem>) -> String {
    let body = generate_table_body(problems);

    return format!(
        r#"
<details>
<summary>{summary}</summary>

| #     | Problem            | Difficulty | Solvings                |
|:-----:|:------------------:|:----------:|:-----------------------:|
{body}
</details>
"#
    );
}

fn generate(problems: HashMap<String, Problem>) -> String {
    let total = problems.len();
    let mut all_problems = problems.values().collect::<Vec<&Problem>>();
    all_problems.sort_by(|x1, x2| x1.id.cmp(&x2.id));
    let all_problems = all_problems.into_iter();

    let all_solvings = generate_solvings_table("All solvings", all_problems.to_owned().collect());

    let hard: Vec<&Problem> = all_problems
        .to_owned()
        .filter(|x| match x.difficult {
            Difficult::Hard => true,
            _ => false,
        })
        .collect();
    let hard_total = hard.len();
    let hard_solvings = generate_solvings_table("Hard", hard);

    let medium: Vec<&Problem> = all_problems
        .to_owned()
        .into_iter()
        .filter(|x| match x.difficult {
            Difficult::Medium => true,
            _ => false,
        })
        .collect();
    let medium_total = medium.len();
    let medium_solvings = generate_solvings_table("Medium", medium);

    let easy: Vec<&Problem> = all_problems
        .to_owned()
        .into_iter()
        .filter(|x| match x.difficult {
            Difficult::Easy => true,
            _ => false,
        })
        .collect();
    let easy_total = easy.len();
    let easy_solvings = generate_solvings_table("Easy", easy);

    return format!(
        r#"# This repo contains my leetcode problem solving tasks

## Number of all solved problems 📈: {total}
{all_solvings}
## Number of "Hard" solved problems 🤯: {hard_total}
{hard_solvings}
## Number of "Medium" solved problems 😨: {medium_total}
{medium_solvings}
## Number of "Easy" solved problems 🥱: {easy_total}
{easy_solvings}

This file was generated automatically by [flaunt](https://github.com/vadimalekseev/flaunt).
"#
    );
}

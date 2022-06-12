fn main() {
    let args = args_to_vec(&mut std::env::args());
    let (dir, excludes) = parse_args(args);

    // println!("dir: {}, excludes: {:?}", dir, excludes);

    let todos = todo::finder::find(&dir, &excludes.unwrap_or_default());

    for todo in todos.unwrap() {
        println!("{}", todo);
    }
}

fn args_to_vec(arg: &mut std::env::Args) -> Vec<String> {
    let mut args = Vec::<String>::new();

    for a in arg {
        args.push(a.clone());
    }

    args
}

fn parse_args(args: Vec<String>) -> (String, Option<Vec<String>>) {
    const EXCLUDE_FLAG: &str = "--exclude=";

    match args.len() {
        0..=1 => ("./".to_string(), None),
        2 => {
            if args[1].len() < EXCLUDE_FLAG.len() || &args[1][0..EXCLUDE_FLAG.len()] != EXCLUDE_FLAG
            {
                return (args[1].clone(), None);
            }

            (
                "./".to_string(),
                Some(parse_excludes(&args[1][EXCLUDE_FLAG.len()..])),
            )
        }
        _ => {
            if args[1].len() < EXCLUDE_FLAG.len() || &args[1][0..EXCLUDE_FLAG.len()] != EXCLUDE_FLAG
            {
                let exclude_arg = &args[2];
                return (
                    args[1].clone(),
                    Some(parse_excludes(&exclude_arg[EXCLUDE_FLAG.len()..])),
                );
            }

            (
                args[2].clone(),
                Some(parse_excludes(&args[1][EXCLUDE_FLAG.len()..])),
            )
        }
    }
}

fn parse_excludes(exclude: &str) -> Vec<String> {
    let mut excludes: Vec<String> = Vec::new();

    for e in exclude.split(",") {
        excludes.push(e.to_string());
    }

    excludes
}

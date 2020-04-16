use clap::{load_yaml, App, Arg, ArgMatches};
use lazy_static::lazy_static;
use maplit::hashmap;
use std::collections::HashMap;
use std::process::Command;
use yaml_rust::Yaml;

lazy_static! {
    // 这里注册 git emoji 列表
    static ref GITMOJI_LIST: Vec<HashMap<&'static str, &'static str>> = vec![
        hashmap! {
            "name" => "feat",
            "emoji" => "✨",
            "code" => ":feat:",
            "description" => "✨ Introduce new features"
        },
        hashmap! {
            "name" => "fix",
            "emoji" => "🐛",
            "code" => ":fix:",
            "description" => "🐛 Fix bugs"
        }
    ];
}

fn main() -> Result<(), std::io::Error> {
    let yml = load_yaml!("cli.yml");
    let matches = make_cli_app(yml).get_matches();
    let commit_config = CommitConfig::from_cli_arg_matches(&matches);
    do_git_commit(&commit_config)
}

fn make_cli_app<'a, 'b>(yml: &'a Yaml) -> App<'a, 'b> {
    let mut app = App::from_yaml(yml);
    // 将 gitmoji 相关命令注册进来
    for item in GITMOJI_LIST.iter() {
        app = app.arg(
            Arg::with_name(item.get("name").unwrap())
                .help(item.get("description").unwrap())
                .long(item.get("name").unwrap()),
        );
    }
    app
}

#[derive(Debug)]
struct CommitConfig {
    message: String,
    use_emoji_code_only: bool,
    gitmoji: Option<Gitmoji>,
}

impl CommitConfig {
    fn from_cli_arg_matches(m: &ArgMatches) -> Self {
        let message = m.value_of("message").unwrap();
        let use_emoji_code_only = m.is_present("use-emoji-code-only");
        dbg!(use_emoji_code_only);
        CommitConfig {
            message: message.to_string(),
            use_emoji_code_only: use_emoji_code_only,
            gitmoji: Self::match_gitmoji(m),
        }
    }

    fn match_gitmoji(m: &ArgMatches) -> Option<Gitmoji> {
        for x in GITMOJI_LIST.iter() {
            if m.is_present(x.get("name").unwrap()) {
                return Some(Gitmoji::from_gitmoji_config(x));
            }
        }
        None
    }
}

#[derive(Debug)]
struct Gitmoji {
    name: String,
    code: String,
    emoji: String,
}

impl Gitmoji {
    fn from_gitmoji_config(c: &HashMap<&str, &str>) -> Self {
        Gitmoji {
            name: c.get("name").unwrap().to_string(),
            code: c.get("code").unwrap().to_string(),
            emoji: c.get("emoji").unwrap().to_string(),
        }
    }
}

fn do_git_commit(c: &CommitConfig) -> Result<(), std::io::Error> {
    let mut msg = c.message.to_string();
    // 构建 commit 消息
    if let Some(gitmoji) = &c.gitmoji {
        msg = match c.use_emoji_code_only {
            true => format!("{code} {message}", code = gitmoji.code, message = c.message),
            false => format!(
                "{emoji} {name}: {message}",
                emoji = gitmoji.emoji,
                name = gitmoji.name,
                message = c.message
            ),
        }
    }

    let mut cmd = Command::new("git");
    cmd.arg("commit").arg("-m").arg(msg);

    // 执行命令，拿到输出结果
    let output = cmd.output()?;
    println!("{}", String::from_utf8_lossy(&output.stdout));
    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

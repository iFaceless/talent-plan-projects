use clap::{load_yaml, App, Arg, ArgMatches};
use lazy_static::lazy_static;
use maplit::hashmap;
use std::collections::HashMap;
use std::process::Command;
use yaml_rust::Yaml;
use std::process;
use std::io::Error;

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

/// 将 gitc 核心业务逻辑串联起来，实现预期的功能。
fn main() {
    let yml = load_yaml!("cli.yml");
    let matches = make_cli_app(yml).get_matches();
    let commit_config = CommitConfig::from_cli_arg_matches(&matches);
    match do_git_commit(&commit_config) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

/// 根据配置文件和可扩展的 emoji 列表配置生成一个 [`clap::App`] 实例。
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
    /// 解析 [`clap::ArgMatches`] 命令行参数结果，并生成 [`CommitConfig`]
    /// 实例，方便后续使用。
    fn from_cli_arg_matches(m: &ArgMatches) -> Self {
        let message = m.value_of("message").unwrap();
        let use_emoji_code_only = m.is_present("use-emoji-code-only");
        CommitConfig {
            message: message.to_string(),
            use_emoji_code_only: use_emoji_code_only,
            gitmoji: Self::match_gitmoji(m),
        }
    }

    /// 通过遍历全局注册的 emoji 列表，匹配用户输入的标签（如 `feat`, `fix` 等）
    /// 如果查找成功，则返回 [`Gitmoji`] 对象。
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
    /// 将输入的 emoji 字典配置转成 [`Gitmoji`] 对象
    fn from_gitmoji_config(c: &HashMap<&str, &str>) -> Self {
        Gitmoji {
            name: c.get("name").unwrap().to_string(),
            code: c.get("code").unwrap().to_string(),
            emoji: c.get("emoji").unwrap().to_string(),
        }
    }
}

/// 根据输入的 [`CommitConfig`] 构建参数，并执行 `git commit -m` 命令。
/// # Errors
/// 如果执行出错，则会返回错误 [`std::io::Error`]。
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

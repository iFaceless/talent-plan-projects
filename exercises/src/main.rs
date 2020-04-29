use fakedata_generator::{gen_email, gen_int, gen_username};
use std::cmp::Ordering;
use std::collections;
use std::error::Error;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    convert_collection_to_another();
    count_words("hello world this is a good day this is a good day");
    store_with_complex_key();
    read_from_terminal_input()?;
    read_file()?;
    Ok(())
}

/// 关于集合类型转换的方法
fn convert_collection_to_another() {
    let v = vec![1, 2, 3, 4, 5];

    // 可以使用 `extend` 进行转换
    let mut v2 = collections::VecDeque::new();
    v2.extend(v.clone());
    dbg!(v2);

    let mut v3 = collections::LinkedList::new();
    v3.extend(v.clone());
    dbg!(v3);

    let mut v4 = Vec::new();
    v4.extend("Hello, world".chars());
    dbg!(v4);

    // 可以使用 `collect` 进行转换
    let v5: Vec<_> = "hello, world".chars().collect();
    dbg!(v5);

    let v6 = vec![1, 2, 3]
        .into_iter()
        .collect::<collections::VecDeque<_>>();
    dbg!(v6);
}

/// 学会使用字典的 Entry API 执行一些需要根据 key
/// 存在性执行的操作
fn count_words(input: &str) {
    let mut m = collections::HashMap::new();
    for word in input.split_whitespace() {
        *m.entry(word).or_insert(0) += 1;
    }

    for (k, v) in m.iter() {
        println!("{} = {}", k, v);
    }
}

#[derive(Debug)]
struct User {
    id: i64,
    name: String,
    email: String,
}

impl User {
    fn new(id: i64, name: String, email: String) -> User {
        User {
            id,
            name: name,
            email: email,
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &User) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}

impl Hash for User {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.id.hash(h)
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &User) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for User {
    fn cmp(&self, other: &User) -> Ordering {
        self.id.cmp(&other.id)
    }
}

fn store_with_complex_key() {
    let mut m = collections::HashMap::new();

    // 在重新更新 Entry 的时候，是不会更新 Key 的哦
    m.insert(
        User::new(1000, "test".to_owned(), "test@xx.com".to_owned()),
        "test".to_owned(),
    );
    m.insert(
        User::new(1000, "test123".to_owned(), "test123@xx.com".to_owned()),
        "test123".to_owned(),
    );

    assert_eq!(m.values().next().unwrap(), "test123");
    assert_eq!(m.keys().next().unwrap().name, "test"); // 注意这里不是 test123

    for u in make_users() {
        let value = format!("[{}] {}-{}", u.id, u.name, u.email);
        m.insert(u, value);
    }
    dbg!(&m);
}

fn make_users() -> Vec<User> {
    let mut users = Vec::new();
    for _ in 0..10 {
        users.push(User::new(
            gen_int("1,100".to_owned()).parse::<i64>().unwrap(),
            gen_username(),
            gen_email(),
        ))
    }

    users
}

// 从终端读取内容
fn read_from_terminal_input() -> Result<(), Box<dyn Error>> {
    let prompt = |msg: &str| -> Result<String, io::Error> {
        println!("{}", msg);
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    };
    let name = prompt("Please input your name: ")?;
    let email = prompt("Please input your email: ")?;
    let user = User::new(
        gen_int("1,100".to_owned()).parse::<_>().unwrap(),
        name.trim().to_owned(),
        email.trim().to_owned(),
    );
    dbg!(user);
    Ok(())
}

fn read_file() -> Result<(), io::Error> {
    let mut f = File::open("src/main.rs")?;

    // 可以直接读取指定字节
    let mut buf = [0; 20];

    // 最多读取 20 个字节
    assert_eq!(true, f.read(&mut buf)? <= 20);
    dbg!(String::from_utf8_lossy(&buf));

    // 精确读取 20 个字节
    assert_eq!(true, f.read(&mut buf)? == 20);
    dbg!(String::from_utf8_lossy(&buf));

    // 使用 BufRead 可以借助内部缓存机制，减少系统调用次数
    let mut rdr = io::BufReader::new(f);
    let mut line = String::new();
    rdr.read_line(&mut line)?;
    dbg!(&line);

    rdr.seek(io::SeekFrom::End(-100))?;
    rdr.read_line(&mut line)?;
    dbg!(&line);

    Ok(())
}


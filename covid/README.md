covid-19 新冠疫情数据查询示例
==============================

```shell
$ caro run -- --help

covid 0.1.0
0xE8551CCB <noti@ifaceless.space>
A handful cli to query covid-19 infections in the world.

USAGE:
    covid [FLAGS] [OPTIONS] <COUNTRY>

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      Don't show noisy messages
    -V, --version    Prints version information

OPTIONS:
    -d, --data-path <DATA_PATH>    Input data file

ARGS:
    <COUNTRY>    Query data of which country
```

使用示例：
```shell
$ cargo run -- -d=assets/covid-19-infections-20200422.csv 美国
Record { country: "美国", number_of_newly_diagnosis: 36386, number_of_cumulative_diagnosis: 825306, number_of_current_diagnosis: 704558, number_of_deaths: 45075, number_of_cures: 75673 }

$ cargo run -- -d=assets/covid-19-infections-20200422.csv 不存在
no matching record found
```
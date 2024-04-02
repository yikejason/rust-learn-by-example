use clap::Parser;

// 定义 HTTPie 的 CLI 的主入口，它包含若干个子命令
// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助

#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcmd: Subcommand,
}

// 子命令分别对应不同的 HTTP 方法，目前支持 get / post
#[derive(Parser, Debug)]
enum Subcommand {
    Get(Get),
    Post(Post),
}

// get 子命令
/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
    /// HTTP 请求的url
    url: String,
}

// post 子命令。需要输入一个 URL, 和若干个可选的 key = value, 用于提供 json body
/// feed post with an url and optional key = value pairs. We will post the data
/// as Json, and retrieve the response for you
#[derive(Parser, Debug)]
struct Post {
    /// HTTP 请求的 URL
    url: String,
    /// HTTP 请求的 body
    body: Vec<String>,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}

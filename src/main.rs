//! Tushare Pro CLI - 主程序入口
//!
//! 获取中国金融市场数据的命令行工具

mod client;
mod cli;
mod error;
mod output;
mod api;
mod config;

use cli::{parse_args, handle_command};

#[tokio::main]
async fn main() {
    // 获取命令行参数
    let args: Vec<String> = std::env::args().collect();

    // 解析参数
    let parsed_args = match parse_args(args) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("参数解析错误: {}", e);
            std::process::exit(1);
        }
    };

    // 处理命令
    if let Err(e) = handle_command(parsed_args).await {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}

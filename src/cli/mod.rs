//! CLI 模块
//!
//! 负责命令行参数解析和命令处理。

pub mod args;
pub mod parser;
pub mod commands;

pub use args::{ParsedArgs, Options, OutputFormat, ParamValue};
pub use parser::parse_args;
pub use commands::handle_command;

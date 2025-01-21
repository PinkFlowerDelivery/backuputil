use clap::Command;

pub mod make;
pub mod clear;

pub fn build_cli() -> Command {
    Command::new("backu")
        .author("PinkFlowerDelivery")
        .version("1.0") 
        .disable_help_flag(true)
        .disable_help_subcommand(true)
        .disable_version_flag(true)
        .arg_required_else_help(true) 
        .help_template("\
{usage-heading} {usage}

{all-args}{after-help}

Author: {author}
Version: {version}
")  
        .subcommand(make::make_command())
        .subcommand(clear::clear_command())
}

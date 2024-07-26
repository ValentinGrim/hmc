use std::process::ExitCode;
use hmc::config;

fn main() -> ExitCode {
    let conf = config::HmcConfig::load();
    return ExitCode::SUCCESS;
}

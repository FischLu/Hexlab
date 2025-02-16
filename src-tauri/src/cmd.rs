use clap::crate_version;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::process::exit;
use strum::IntoEnumIterator;
use anyhow::Result;

use crate::{
    format::{FormatRadix, OutputFormat},
    options::Options,
    error,
    expression,
    config::Config,
};

pub fn cmd_main(mut config: Config, options: Options) {
    config.override_from_options(&options);

    if let Some(expr_vec) = &options.expr {
        let expr_str = expr_vec.join(" ");
        inline_evaluate(&expr_str, &config, &options);
    } else if let Some(file_path) = &options.file {
        script_evaluate(file_path, &mut config);
    } else if options.interactive {
        interactive(&mut config);
    }
}

fn script_evaluate(file_path: &str, config: &mut Config) {
    let file = File::open(file_path);

    let file = match file {
        Ok(file) => file,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };

    let lines = io::BufReader::new(file).lines();

    let mut ans = 0;
    let mut of = OutputFormat::default()
        .with_format_radix(*config.output_radix())
        .with_punctuate_number(*config.punctuate_output());

    for line in lines {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                eprintln!("{}", err);
                exit(1);
            }
        };
        match proccess_command(line, &mut ans, &mut of, config) {
            Ok(_) => continue,
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        };
    }
}

fn inline_evaluate(expr_str: &str, config: &Config, options: &Options) {
    match expression::parse_line(expr_str, config) {
        Ok(command) => match command {
            expression::Command::Expr(expr) => match expression::eval::eval_expr(&expr, 0) {
                Ok(ans) => {
                    if options.all {
                        for radix in FormatRadix::iter() {
                            println!(
                                "{:>21}: {}",
                                radix.to_string(),
                                OutputFormat::default()
                                    .with_format_radix(radix)
                                    .with_punctuate_number(*config.punctuate_output())
                                    .fmt(ans),
                            );
                        }
                    } else {
                        println!(
                            "{}",
                            OutputFormat::default()
                                .with_format_radix(*config.output_radix())
                                .with_punctuate_number(*config.punctuate_output())
                                .fmt(ans),
                        );
                    }
                }
                Err(err) => {
                    eprintln!("Failed to evaluate \"{}\": {}", expr_str, err);
                    exit(1);
                }
            },
            expression::Command::Set(_) => {
                eprintln!("Set directive not allowed in inline-expression");
                exit(1);
            }
            expression::Command::Convert(conversion) => match conversion.value(0) {
                Ok(ans) => {
                    if options.all {
                        for radix in FormatRadix::iter() {
                            println!(
                                "{:>21}: {}",
                                radix.to_string(),
                                OutputFormat::default()
                                    .with_format_radix(radix)
                                    .with_punctuate_number(*config.punctuate_output())
                                    .fmt(ans),
                            );
                        }
                    } else {
                        println!(
                            "{}",
                            OutputFormat::default()
                                .with_format_radix(conversion.radix())
                                .with_punctuate_number(*config.punctuate_output())
                                .fmt(ans),
                        );
                    }
                }
                Err(err) => {
                    eprintln!("Failed to evaluate \"{}\": {}", expr_str, err);
                    exit(1);
                }
            },
            expression::Command::Empty => {
                println!("Empty expression!")
            }
        },
        Err(err) => {
            eprintln!("Failed to parse \"{}\": {}", expr_str, err);
            exit(1);
        }
    }
}

fn interactive(config: &mut Config) {
    if *config.header() {
        welcome(config);
    }

    let Ok(mut rl) = DefaultEditor::new() else {
        eprintln!("Failed to create rustyline editor");
        exit(1);
    };
    let history_file_name = PathBuf::from(".cork_history");
    let home_dir = home::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let mut history_path = home_dir;
    history_path.push(history_file_name);

    if rl.load_history(&history_path).is_err() {
        println!("No existing history!\n");
    } else {
        println!();
    }

    let mut of = OutputFormat::default()
        .with_format_radix(*config.output_radix())
        .with_punctuate_number(*config.punctuate_output());
    let mut ans = 0;
    loop {
        match rl.readline(config.prompt()) {
            Ok(line) => {
                let _ = rl.add_history_entry(&line);
                match proccess_command(line, &mut ans, &mut of, config) {
                    Ok(_) => continue,
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                };
            }
            Err(ReadlineError::Eof) => {
                println!("Exiting ... ");
                break;
            }
            Err(ReadlineError::Interrupted) => {
                println!("Exiting ... ");
                break;
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
    if *config.history() {
        rl.save_history(&history_path).unwrap();
    }
}

fn proccess_command(line: String, ans: &mut i64, of: &mut OutputFormat, config: &mut Config) -> Result<()> {
    let command = expression::parse_line(&line, config)?;
    match command {
        expression::Command::Expr(expr) => {
            let val = expression::eval::eval_expr(&expr, *ans)?;
            *ans = val;
            println!("{}", of.fmt(val));
        }
        expression::Command::Set(set) => {
            if set[0] == "of" {
                match set[1].as_str() {
                    "hex" => of.set_format_radix(FormatRadix::Hex),
                    "dec" => of.set_format_radix(FormatRadix::Decimal),
                    "oct" => of.set_format_radix(FormatRadix::Octal),
                    "bin" => of.set_format_radix(FormatRadix::Binary),
                    _ => {
                        return Err(error::CorkError::InvalidValueForKey {
                            key: set[0].clone(),
                            value: set[1].clone(),
                        }.into());
                    }
                }
            } else if set[0] == "mode" {
                match set[1].as_str() {
                    "hex" => {config.set_mode("hex".to_string());},
                    "dec" => {config.set_mode("dec".to_string());},
                    _ => {
                        return Err(error::CorkError::InvalidValueForKey {
                            key: set[0].clone(),
                            value: set[1].clone(),
                        }.into());
                    }
                }
            } else {
                return Err(error::CorkError::InvalidKey(set[0].clone()).into());
            }
        }
        expression::Command::Convert(conversion) => {
            let val = conversion.value(*ans)?;
            *ans = val;
            println!(
                "{}",
                OutputFormat::default()
                    .with_format_radix(conversion.radix())
                    .with_punctuate_number(of.punctuate_number())
                    .fmt(val)
            );
        }
        expression::Command::Empty => println!(),
    };
    Ok(())
}

fn welcome(config: &Config) {
    println!("Cork, version {}", crate_version!());
    // println!("Welcome to cork - a calculator for hex-lovers!");
    println!("Current mode: {}", config.mode());
    println!("Press Ctrl + D or Ctrl + C to exit.");
}
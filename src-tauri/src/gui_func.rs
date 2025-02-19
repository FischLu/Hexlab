use serde::Deserialize;
use crate::{
    format::{FormatRadix, OutputFormat},
    expression,
    CONFIG,
};

// Deserialize is needed to accept parameter from TS
#[derive(Debug, Deserialize)]
pub struct CalcOptions {
    pub mode: String,
}

#[tauri::command]
pub fn evaluate_expression(expr_str: &str, options: CalcOptions) -> Result<String, String> {
    let mut config = CONFIG.lock().unwrap();
    config.set_mode(options.mode);
    match expression::parse_line(expr_str, &config) {
        Ok(command) => match command {
            expression::Command::Expr(expr) => match expression::eval::eval_expr(&expr, 0) {
                Ok(ans) => {
                    Ok(OutputFormat::default()
                        .with_format_radix(FormatRadix::Hex)
                        .with_punctuate_number(*config.punctuate_output())
                        .fmt(ans))
                }
                Err(err) => Err(format!("Failed to evaluate \"{}\": {}", expr_str, err)),
            },
            expression::Command::Set(_) => Err("Set directive not allowed in inline-expression".to_string()),
            expression::Command::Convert(conversion) => match conversion.value(0) {
                Ok(ans) => {
                    Ok(OutputFormat::default()
                        .with_format_radix(FormatRadix::Hex)
                        .with_punctuate_number(*config.punctuate_output())
                        .fmt(ans))
                }
                Err(err) => Err(format!("Failed to evaluate \"{}\": {}", expr_str, err)),
            },
            expression::Command::Empty => Ok("Empty expression!".to_string()),
        },
        // Err(err) => Err(format!("Failed to parse \"{}\": {}", expr_str, err)),
        Err(err) => Err(format!("{}", err)),
    }
}
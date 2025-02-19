use super::eval::*;
use super::*;
use anyhow::Error;

impl Expr {
    pub fn new_num(value: i64) -> Self {
        Expr::Num(value, Radix::Dec)
    }
}

#[test]
fn test_expr_parse() {
    let mut config: Config = Config::new();
    config.set_mode("dec".to_string());
    let expr1 = Expr::BinOp(BinOpExpr {
        left: Box::new(Expr::BinOp(BinOpExpr {
            left: Box::new(Expr::new_num(5)),
            right: Box::new(Expr::new_num(6)),
            op: Op::Add,
        })),
        right: Box::new(Expr::new_num(2)),
        op: Op::Mul,
    });
    let expr_str1 = "(5 + 6) * 2";
    assert_eq!(parse_line(expr_str1, &config).unwrap(), Command::Expr(expr1));
    let expr2 = Expr::BinOp(BinOpExpr {
        right: Box::new(Expr::BinOp(BinOpExpr {
            left: Box::new(Expr::new_num(5)),
            right: Box::new(Expr::new_num(6)),
            op: Op::Add,
        })),
        left: Box::new(Expr::new_num(2)),
        op: Op::Mul,
    });
    let expr_str2 = "2 * (5 + 6)";
    assert_eq!(parse_line(expr_str2, &config).unwrap(), Command::Expr(expr2));

    config.set_mode("hex".to_string());
    let expr1 = Expr::BinOp(BinOpExpr {
        left: Box::new(Expr::BinOp(BinOpExpr {
            left: Box::new(Expr::Num(5, Radix::Hex)),
            right: Box::new(Expr::Num(6, Radix::Hex)),
            op: Op::Add,
        })),
        right: Box::new(Expr::Num(2, Radix::Hex)),
        op: Op::Mul,
    });
    let expr_str1 = "(5 + 6) * 2";
    assert_eq!(parse_line(expr_str1, &config).unwrap(), Command::Expr(expr1));
    let expr2 = Expr::BinOp(BinOpExpr {
        right: Box::new(Expr::BinOp(BinOpExpr {
            left: Box::new(Expr::Num(5, Radix::Hex)),
            right: Box::new(Expr::Num(6, Radix::Hex)),
            op: Op::Add,
        })),
        left: Box::new(Expr::Num(2, Radix::Hex)),
        op: Op::Mul,
    });
    let expr_str2 = "2 * (5 + 6)";
    assert_eq!(parse_line(expr_str2, &config).unwrap(), Command::Expr(expr2));
}

#[test]
fn test_expr_eval() {
    let mut config: Config = Config::new();
    config.set_mode("dec".to_string());
    let expr1_str = "(5 + 6) * 2";
    match parse_line(expr1_str, &config).unwrap() {
        Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 22),
        _ => panic!("Should have parsed to an expr"),
    };
    let expr2_str = "2 * (5 + 6)";
    match parse_line(expr2_str, &config).unwrap() {
        Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 22),
        _ => panic!("Should have parsed to an expr"),
    };
    let expr3_str = "3 * (9 + 6) - 4";
    match parse_line(expr3_str, &config).unwrap() {
        Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 41),
        _ => panic!("Should have parsed to an expr"),
    };
    let expr4_str = "6-57*(18+4/73)+38 *  124";
    match parse_line(expr4_str, &config).unwrap() {
        Command::Expr(expr) => {
            assert_eq!(eval_expr(&expr, 0).unwrap(), 3692)
        }
        _ => panic!("Should have parsed to an expr"),
    };
    let expr5_str = "2 + (((7 * 2) - 4) / 2) + 8 * 9 / 4";
    match parse_line(expr5_str, &config).unwrap() {
        Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 25),
        _ => panic!("Should have parsed to an expr"),
    };
    let expr6_str = "(3 + 2) - 1 / 1 * 3 + 5 * 4 / 10 - 1";
    match parse_line(expr6_str, &config).unwrap() {
        Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 3),
        _ => panic!("Should have parsed to an expr"),
    };
    let expr7_str = "8 / 2 * 3 - 9 - 6 * (15 / 3 / 5)";
    match parse_line(expr7_str, &config).unwrap() {
        Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), -3),
        _ => panic!("Should have parsed to an expr"),
    };
    let expr8_str = "24 / (2 * (12 / 4)) - ((8 * 3) / 6)";
    match parse_line(expr8_str, &config).unwrap() {
        Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 0),
        _ => panic!("Should have parsed to an expr"),
    };
    let expr9_str = "3 * 512 >> 4 - 2";
    match parse_line(expr9_str, &config).unwrap() {
        Command::Expr(expr) => {
            assert_eq!(eval_expr(&expr, 0).unwrap(), 384)
        }
        _ => panic!("Should have parsed to an expr"),
    };
    let expr10_str = "3 * (512 >> 4) - 2";
    match parse_line(expr10_str, &config).unwrap() {
        Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 94),
        _ => panic!("Should have parsed to an expr"),
    };
    // testing just the bitwise AND
    let expr11_str = "0b0011 & 0b0110";
    match parse_line(expr11_str, &config).unwrap() {
        Command::Expr(expr) => {
            assert_eq!(eval_expr(&expr, 0).unwrap(), 0b0010)
        }
        _ => panic!("Should have parsed to an expr"),
    }
    // testing just the bitwise OR
    let expr12_str = "0b0011 | 0b0110";
    match parse_line(expr12_str, &config).unwrap() {
        Command::Expr(expr) => {
            assert_eq!(eval_expr(&expr, 0).unwrap(), 0b0111)
        }
        _ => panic!("Should have parsed to an expr"),
    }
    // testing just the bitwise XOR
    let expr13_str = "0b0011 ^ 0b0101";
    match parse_line(expr13_str, &config).unwrap() {
        Command::Expr(expr) => {
            assert_eq!(eval_expr(&expr, 0).unwrap(), 0b0110)
        }
        _ => panic!("Should have parsed to an expr"),
    }
    // testing all of the bitwise operators together
    let expr14_str = "((0b0011 ^ 0b0101) & 0b0011) | 0b0111";
    match parse_line(expr14_str, &config).unwrap() {
        Command::Expr(expr) => {
            assert_eq!(eval_expr(&expr, 0).unwrap(), 0b0111)
        }
        _ => panic!("Should have parsed to an expr"),
    }
    // mixing bitwise and "normal" operators
    let expr15_str = "(((0b0011 ^ 0b0101) * 2) & 0b0101) + 1";
    match parse_line(expr15_str, &config).unwrap() {
        Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 5),
        _ => panic!("Should have parsed to an expr"),
    }
    // testing operator precedence / priority with bitwise ops
    let expr16_str = "0b0100 ^ 0b0000 | 0b0101 * 2 & 0b0101 + 1";
    match parse_line(expr16_str, &config).unwrap() {
        Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 6),
        _ => panic!("Should have parsed to an expr"),
    }

    config.set_mode("hex".to_string());
    let expr1_str = "(5 + 6) * 2";
    match parse_line(expr1_str, &config).unwrap() {
        Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 0x16),
        _ => panic!("Should have parsed to an expr"),
    };
    let expr2_str = "2 * (5 + 6)";
    match parse_line(expr2_str, &config).unwrap() {
        Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 0x16),
        _ => panic!("Should have parsed to an expr"),
    };
    let expr3_str = "3 * (9 + 6) - 4";
    match parse_line(expr3_str, &config).unwrap() {
        Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 0x29),
        _ => panic!("Should have parsed to an expr"),
    };
    let expr4_str = "f * a + 5 - 2";
    match parse_line(expr4_str, &config).unwrap() {
        Command::Expr(expr) => {
            assert_eq!(eval_expr(&expr, 0).unwrap(), 0x99)
        }
        _ => panic!("Should have parsed to an expr"),
    };
}

#[test]
fn test_convert_parse() {
    let mut config: Config = Config::new();
    config.set_mode("dec".to_string());
    let conv_str = "(5 + 6) * 2 to hex";
    let conv1 = ConvDirective {
        expr: Expr::BinOp(BinOpExpr {
            left: Box::new(Expr::BinOp(BinOpExpr {
                left: Box::new(Expr::new_num(5)),
                right: Box::new(Expr::new_num(6)),
                op: Op::Add,
            })),
            right: Box::new(Expr::new_num(2)),
            op: Op::Mul,
        }),
        radix: FormatRadix::Hex,
    };
    assert_eq!(parse_line(conv_str, &config).unwrap(), Command::Convert(conv1));

    let conv2 = ConvDirective {
        expr: Expr::BinOp(BinOpExpr {
            left: Box::new(Expr::BinOp(BinOpExpr {
                left: Box::new(Expr::new_num(5)),
                right: Box::new(Expr::new_num(6)),
                op: Op::Add,
            })),
            right: Box::new(Expr::new_num(2)),
            op: Op::Mul,
        }),
        radix: FormatRadix::Decimal,
    };
    assert_ne!(parse_line(conv_str, &config).unwrap(), Command::Convert(conv2));

    let conv3 = ConvDirective {
        expr: Expr::BinOp(BinOpExpr {
            left: Box::new(Expr::BinOp(BinOpExpr {
                left: Box::new(Expr::new_num(5)),
                right: Box::new(Expr::new_num(6)),
                op: Op::Add,
            })),
            right: Box::new(Expr::new_num(2)),
            op: Op::Mul,
        }),
        radix: FormatRadix::Octal,
    };
    assert_ne!(parse_line(conv_str, &config).unwrap(), Command::Convert(conv3));

    let conv4 = ConvDirective {
        expr: Expr::BinOp(BinOpExpr {
            left: Box::new(Expr::BinOp(BinOpExpr {
                left: Box::new(Expr::new_num(5)),
                right: Box::new(Expr::new_num(6)),
                op: Op::Add,
            })),
            right: Box::new(Expr::new_num(2)),
            op: Op::Mul,
        }),
        radix: FormatRadix::Binary,
    };
    assert_ne!(parse_line(conv_str, &config).unwrap(), Command::Convert(conv4));

    config.set_mode("hex".to_string());
    let conv1 = ConvDirective {
        expr: Expr::BinOp(BinOpExpr {
            left: Box::new(Expr::BinOp(BinOpExpr {
                left: Box::new(Expr::Num(5, Radix::Hex)),
                right: Box::new(Expr::Num(6, Radix::Hex)),
                op: Op::Add,
            })),
            right: Box::new(Expr::Num(2, Radix::Hex)),
            op: Op::Mul,
        }),
        radix: FormatRadix::Hex,
    };
    assert_eq!(parse_line(conv_str, &config).unwrap(), Command::Convert(conv1));

    let conv2 = ConvDirective {
        expr: Expr::BinOp(BinOpExpr {
            left: Box::new(Expr::BinOp(BinOpExpr {
                left: Box::new(Expr::Num(5, Radix::Hex)),
                right: Box::new(Expr::Num(6, Radix::Hex)),
                op: Op::Add,
            })),
            right: Box::new(Expr::Num(2, Radix::Hex)),
            op: Op::Mul,
        }),
        radix: FormatRadix::Decimal,
    };
    assert_ne!(parse_line(conv_str, &config).unwrap(), Command::Convert(conv2));

    let conv3 = ConvDirective {
        expr: Expr::BinOp(BinOpExpr {
            left: Box::new(Expr::BinOp(BinOpExpr {
                left: Box::new(Expr::Num(5, Radix::Hex)),
                right: Box::new(Expr::Num(6, Radix::Hex)),
                op: Op::Add,
            })),
            right: Box::new(Expr::Num(2, Radix::Hex)),
            op: Op::Mul,
        }),
        radix: FormatRadix::Octal,
    };
    assert_ne!(parse_line(conv_str, &config).unwrap(), Command::Convert(conv3));

    let conv4 = ConvDirective {
        expr: Expr::BinOp(BinOpExpr {
            left: Box::new(Expr::BinOp(BinOpExpr {
                left: Box::new(Expr::Num(5, Radix::Hex)),
                right: Box::new(Expr::Num(6, Radix::Hex)),
                op: Op::Add,
            })),
            right: Box::new(Expr::Num(2, Radix::Hex)),
            op: Op::Mul,
        }),
        radix: FormatRadix::Binary,
    };
    assert_ne!(parse_line(conv_str, &config).unwrap(), Command::Convert(conv4));
}

#[test]
fn test_tor_directive_eval() {
    let config: Config = Config::new();
    // Checks invariance for the 'dec' conversion
    let expr1_str = "(5 + 6) * 2 to dec";
    match parse_line(expr1_str, &config).unwrap() {
        Command::Convert(conv) => {
            assert_eq!(eval_expr(&conv.expr, 0).unwrap(), 22)
        }
        _ => panic!("Should have parsed to a conversion"),
    };

    // Checks invariance for the 'bin' conversion
    let expr2_str = "(5 + 6) * 2 to bin";
    match parse_line(expr2_str, &config).unwrap() {
        Command::Convert(conv) => {
            assert_eq!(eval_expr(&conv.expr, 0).unwrap(), 22)
        }
        _ => panic!("Should have parsed to a conversion"),
    };

    // Checks invariance for the 'hex' conversion
    let expr3_str = "(5 + 6) * 2 to hex";
    match parse_line(expr3_str, &config).unwrap() {
        Command::Convert(conv) => {
            assert_eq!(eval_expr(&conv.expr, 0).unwrap(), 22)
        }
        _ => panic!("Should have parsed to a conversion"),
    };

    // Checks invariance for the 'oct' conversion
    let expr4_str = "(5 + 6) * 2 to oct";
    match parse_line(expr4_str, &config).unwrap() {
        Command::Convert(conv) => {
            assert_eq!(eval_expr(&conv.expr, 0).unwrap(), 22)
        }
        _ => panic!("Should have parsed to a conversion"),
    };

    use pest::error::{Error as PestError, ErrorVariant as PestVariant};
    use pest::Position;

    let expr5_str = "(5 + 6) * 2 to nonex";
    let result = parse_line(expr5_str, &config).unwrap_err();
    let expected: Error = PestError::new_from_pos(
        PestVariant::ParsingError {
            positives: vec![Rule::radix],
            negatives: vec![],
        },
        Position::new(expr5_str, 15).unwrap(),
    ).into();

    let not_expected: Error = PestError::new_from_pos(
        PestVariant::ParsingError {
            positives: vec![Rule::EOI],
            negatives: vec![],
        },
        Position::new(expr5_str, 18).unwrap(),
    ).into();

    // Checks error on non-existent radix, must return a wrong radix error
    assert_eq!(expected.to_string(), result.to_string());

    // Checks error on non-existent radix, must not return an 'expected EOI' error
    assert_ne!(not_expected.to_string(), result.to_string());

    let expr6_str = "(5 + 6) * 2 to decimal";
    let result = parse_line(expr6_str, &config).unwrap_err();
    let expected: Error = PestError::new_from_pos(
        PestVariant::ParsingError {
            positives: vec![Rule::EOI],
            negatives: vec![],
        },
        Position::new(expr6_str, 18).unwrap(),
    ).into();

    let not_expected: Error = PestError::new_from_pos(
        PestVariant::ParsingError {
            positives: vec![Rule::radix],
            negatives: vec![],
        },
        Position::new(expr6_str, 15).unwrap(),
    ).into();

    // Checks error on a missing EOI after the radix, must return an 'expected EOI' error
    assert_eq!(expected.to_string(), result.to_string());

    // Checks error on a missing EOI after the radix, must not return a wrong radix error
    assert_ne!(not_expected.to_string(), result.to_string());
}

#[test]
fn test_convert_output() {
    let mut config: Config = Config::new();
    config.set_mode("dec".to_string());
    use crate::format::OutputFormat;

    let expr_dec = "127 to dec";
    match parse_line(expr_dec, &config).unwrap() {
        Command::Convert(conversion) => {
            let result = format!(
                "{:?}",
                OutputFormat::default()
                    .with_format_radix(conversion.radix())
                    .fmt(conversion.value(0).unwrap())
            );
            assert_eq!(result, "\"0d127\"");
        }
        _ => panic!("Should have parsed to a conversion"),
    };

    let expr_hex = "127 to hex";
    match parse_line(expr_hex, &config).unwrap() {
        Command::Convert(conversion) => {
            let result = format!(
                "{:?}",
                OutputFormat::default()
                    .with_format_radix(conversion.radix())
                    .fmt(conversion.value(0).unwrap())
            );
            assert_eq!(result, "\"0x7f\"");
        }
        _ => panic!("Should have parsed to a conversion"),
    };

    let expr_oct = "127 to oct";
    match parse_line(expr_oct, &config).unwrap() {
        Command::Convert(conversion) => {
            let result = format!(
                "{:?}",
                OutputFormat::default()
                    .with_format_radix(conversion.radix())
                    .fmt(conversion.value(0).unwrap())
            );
            assert_eq!(result, "\"0o177\"");
        }
        _ => panic!("Should have parsed to a conversion"),
    };

    let expr_bin = "127 to bin";
    match parse_line(expr_bin, &config).unwrap() {
        Command::Convert(conversion) => {
            let result = format!(
                "{:?}",
                OutputFormat::default()
                    .with_format_radix(conversion.radix())
                    .fmt(conversion.value(0).unwrap())
            );
            assert_eq!(result, "\"0b111_1111\"");
        }
        _ => panic!("Should have parsed to a conversion"),
    };

    config.set_mode("hex".to_string());
    let expr_dec = "127 to dec";
    match parse_line(expr_dec, &config).unwrap() {
        Command::Convert(conversion) => {
            let result = format!(
                "{:?}",
                OutputFormat::default()
                    .with_format_radix(conversion.radix())
                    .fmt(conversion.value(0).unwrap())
            );
            assert_eq!(result, "\"0d295\"");
        }
        _ => panic!("Should have parsed to a conversion"),
    };

    let expr_hex = "127 to hex";
    match parse_line(expr_hex, &config).unwrap() {
        Command::Convert(conversion) => {
            let result = format!(
                "{:?}",
                OutputFormat::default()
                    .with_format_radix(conversion.radix())
                    .fmt(conversion.value(0).unwrap())
            );
            assert_eq!(result, "\"0x127\"");
        }
        _ => panic!("Should have parsed to a conversion"),
    };
}

#[test]
fn hex_parse() {
    let config: Config = Config::new();
    let hex_str1 = "0x1a";
    assert_eq!(
        parse_line(hex_str1, &config).unwrap(), 
        Command::Convert(ConvDirective { expr: Expr::Num(26, Radix::HexWithPrefix), radix: FormatRadix::Decimal })
    );
    let hex_str2 = "0xCAFE";
    assert_eq!(
        parse_line(hex_str2, &config).unwrap(),
        Command::Convert(ConvDirective { expr: Expr::Num(51966, Radix::HexWithPrefix), radix: FormatRadix::Decimal })
    );
    let hex_str3 = "0xFACE_A0CE";
    assert_eq!(
        parse_line(hex_str3, &config).unwrap(),
        Command::Convert(ConvDirective { expr: Expr::Num(4207845582, Radix::HexWithPrefix), radix: FormatRadix::Decimal })
    );
}

#[test]
fn oct_parse() {
    let config: Config = Config::new();
    let oct_str1 = "0o345";
    assert_eq!(
        parse_line(oct_str1, &config).unwrap(), 
        Command::Convert(ConvDirective { expr: Expr::Num(229, Radix::Oct), radix: FormatRadix::Decimal })
    );
    let oct_str2 = "0o1232344";
    assert_eq!(
        parse_line(oct_str2, &config).unwrap(),
        Command::Convert(ConvDirective { expr: Expr::Num(341220, Radix::Oct), radix: FormatRadix::Decimal })
    );
    let oct_str3 = "0o1232_34_4";
    assert_eq!(
        parse_line(oct_str3, &config).unwrap(),
        Command::Convert(ConvDirective { expr: Expr::Num(341220, Radix::Oct), radix: FormatRadix::Decimal })
    );
}

#[test]
fn bin_parse() {
    let config: Config = Config::new();
    let bin_str1 = "0b1010";
    assert_eq!(
        parse_line(bin_str1, &config).unwrap(), 
        Command::Convert(ConvDirective { expr: Expr::Num(10, Radix::Bin), radix: FormatRadix::Decimal })
    );
    let bin_str1 = "0b10100101";
    assert_eq!(
        parse_line(bin_str1, &config).unwrap(), 
        Command::Convert(ConvDirective { expr: Expr::Num(165, Radix::Bin), radix: FormatRadix::Decimal })
    );
    let bin_str3 = "0b10_10_01____01";
    assert_eq!(
        parse_line(bin_str3, &config).unwrap(), 
        Command::Convert(ConvDirective { expr: Expr::Num(165, Radix::Bin), radix: FormatRadix::Decimal })
    );
}

#[test]
fn dec_parse() {
    let mut config: Config = Config::new();
    config.set_mode("dec".to_string());
    let dec_str1 = "1234_5678";
    assert_eq!(
        parse_line(dec_str1, &config).unwrap(),
        Command::Convert(ConvDirective { expr: Expr::Num(12345678, Radix::Dec), radix: FormatRadix::Hex })
    );
    let dec_str2 = "0d1234_5678";
    assert_eq!(
        parse_line(dec_str2, &config).unwrap(),
        Command::Convert(ConvDirective { expr: Expr::Num(12345678, Radix::DecWithPrefix), radix: FormatRadix::Hex })
    );

    config.set_mode("hex".to_string());
    let dec_str1 = "1234_5678";
    assert_eq!(
        parse_line(dec_str1, &config).unwrap(),
        Command::Convert(ConvDirective { expr: Expr::Num(305419896, Radix::Hex), radix: FormatRadix::Decimal })
    );
    let dec_str2 = "0d1234_5678";
    assert_eq!(
        parse_line(dec_str2, &config).unwrap(),
        Command::Convert(ConvDirective { expr: Expr::Num(12345678, Radix::DecWithPrefix), radix: FormatRadix::Hex })
    );
}


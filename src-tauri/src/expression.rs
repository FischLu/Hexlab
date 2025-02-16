use crate::error::CorkError;
use crate::format::FormatRadix;
use once_cell::sync::Lazy;
use pest::iterators::{Pair, Pairs};
use pest::pratt_parser::PrattParser;
use pest::Parser;
use pest_derive::Parser;
use std::fmt;
use std::num::ParseIntError;
use std::ops::Index;
use std::str::FromStr;
use anyhow::{Result, Context};
use crate::Config;

#[cfg(test)]
mod expression_test;

#[derive(Parser)]
#[grammar = "expression.peg"]
struct CommandParser;

use pest::error::Error as PestError;
pub(crate) type PestRuleError = PestError<Rule>;

/// An Expr is either a node (which corresponds to a binary operation) or a leaf (which corresponds
/// to a number).
#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    BinOp(BinOpExpr),
    Num(i64, Radix),
    Ans,
}

/// An Op is a binary operator.
#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    Xor,
    LShift,
    RShift,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ParseOpError(String);

impl FromStr for Op {
    type Err = ParseOpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            "%" => Ok(Op::Rem),
            "&" => Ok(Op::And),
            "|" => Ok(Op::Or),
            "^" => Ok(Op::Xor),
            "<<" => Ok(Op::LShift),
            ">>" => Ok(Op::RShift),
            _ => Err(ParseOpError(format!("{} is not an Op", s))),
        }
    }
}

/// A BinOpExpr is an expr which has two operands and an operator.
/// The two operands might also be expressions.
#[derive(Debug, Eq)]
pub struct BinOpExpr {
    left: Box<Expr>,
    right: Box<Expr>,
    op: Op,
}

impl PartialEq for BinOpExpr {
    fn eq(&self, rhs: &Self) -> bool {
        *self.left == *rhs.left && *self.right == *rhs.right && self.op == rhs.op
    }
}

/// A SetDirective is a command of the form "set [args]+".
#[derive(Debug, PartialEq, Eq)]
pub struct SetDirective {
    args: Vec<String>,
}

impl fmt::Display for SetDirective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "set {}", self.args.join(" "))
    }
}

impl Index<usize> for SetDirective {
    type Output = String;

    fn index(&self, idx: usize) -> &String {
        &self.args[idx]
    }
}

/// A ConvDirective is a command of the form "to hex|dec|bin|oct|dec".
#[derive(Debug, PartialEq, Eq)]
pub struct ConvDirective {
    expr: Expr,
    radix: FormatRadix,
}

impl ConvDirective {
    pub fn value(&self, ans: i64) -> Result<i64, CorkError> {
        eval::eval_expr(&self.expr, ans)
    }

    pub fn radix(&self) -> FormatRadix {
        self.radix
    }
}

impl fmt::Display for ConvDirective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "convert to {}", &self.radix)
    }
}

/// A Command is a one line worth of input from the user.
/// It can either be a SetDirective or an Expr.
/// As an escape-hatch, there is also an empty command.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Expr(Expr),
    Set(SetDirective),
    Convert(ConvDirective),
    Empty,
}

/// parse_line takes in a user input, and parses it to a valid Command
/// or results in a parse error.
pub fn parse_line<T: AsRef<str>>(line: T, config: &Config) -> Result<Command> {
    let mut pairs = if config.mode() == "hex" {
        CommandParser::parse(Rule::line_hex, line.as_ref())?
    } else if config.mode() == "dec" {
        CommandParser::parse(Rule::line_dec, line.as_ref())?
    } else {
        unreachable!("mode {} is not supported!", config.mode())
    };
    
    let comm = pairs.next();
    if comm.is_none() {
        return Ok(Command::Empty);
    }
    parse_comm(comm.unwrap().into_inner().next().unwrap())
}

fn parse_comm(pair: Pair<Rule>) -> Result<Command> {
    match pair.as_rule() {
        Rule::expr_dec | Rule::expr_hex => Ok(Command::Expr(parse_expr(pair.into_inner())?)),
        Rule::set_directive => Ok(Command::Set(SetDirective {
            args: pair.as_str().split(' ').skip(1).map(String::from).collect(),
        })),
        Rule::tor_directive_dec | Rule::tor_directive_hex => {
            let mut pairs = pair.into_inner();
            let expr_pair = pairs.next().unwrap();
            let radix_pair = pairs.next().unwrap();
            Ok(Command::Convert(ConvDirective {
                expr: parse_expr(expr_pair.into_inner())?,
                radix: parse_radix(radix_pair),
            }))
        }
        Rule::convert_directive_dec | Rule::convert_directive_hex => {
            match parse_expr(pair.into_inner())? {
                Expr::Num(num, radix)  => {
                    match radix {
                        Radix::DecWithPrefix | Radix::Dec => {
                            Ok(Command::Convert(ConvDirective {
                                expr: Expr::Num(num, radix),
                                radix: FormatRadix::Hex,
                            }))
                        },
                        Radix::HexWithPrefix | Radix::Hex => {
                            Ok(Command::Convert(ConvDirective {
                                expr: Expr::Num(num, radix),
                                radix: FormatRadix::Decimal,
                            }))
                        },
                        _ => {
                            Ok(Command::Convert(ConvDirective {
                                expr: Expr::Num(num, radix),
                                radix: FormatRadix::Decimal,
                            }))
                        },
                    }
                },
                rule => unreachable!("only support single number input to convert, found {:?}", rule),
            }
        }
        _ => unreachable!(),
    }
}

static PRATT_PARSER: Lazy<PrattParser<Rule>> = Lazy::new(|| {
    use pest::pratt_parser::{Assoc::*, Op};
    use Rule::*;

    // The operators are defined in an increasing order of precedence
    // Operators at the same level have same precedence
    // The "Left" indicates that the operators associate to the left
    PrattParser::new()
        .op(Op::infix(or, Left))
        .op(Op::infix(xor, Left))
        .op(Op::infix(and, Left))
        .op(Op::infix(lshift, Left) | Op::infix(rshift, Left))
        .op(Op::infix(add, Left) | Op::infix(subtract, Left))
        .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(rem, Left))
});

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Radix {
    Bin,
    Oct,
    Dec,
    DecWithPrefix,
    Hex,
    HexWithPrefix,
}

impl Radix {
    fn numeric_radix(&self) -> u32 {
        match self {
            Radix::Bin => 2,
            Radix::Oct => 8,
            Radix::Dec => 10,
            Radix::DecWithPrefix => 10,
            Radix::Hex => 16,
            Radix::HexWithPrefix => 16,
        }
    }
}

fn parse_radix(p: Pair<Rule>) -> FormatRadix {
    match p.as_str() {
        "dec" => FormatRadix::Decimal,
        "oct" => FormatRadix::Octal,
        "hex" => FormatRadix::Hex,
        "bin" => FormatRadix::Binary,
        _ => unreachable!(),
    }
}

fn parse_num(mut s: &str, radix: Radix) -> Result<i64, ParseIntError> {
    // Check for a negative sign
    let negative = s.starts_with('-');
    if negative {
        s = &s[1..];
    }
    // For numbers with a prefix (for rules other than Dec/Hex), remove the prefix (length 2, e.g., "0d", "0x", "0o", "0b")
    if radix != Radix::Dec && radix != Radix::Hex {
        s = &s[2..];
    }
    // Remove any underscores for readability
    let num_str = s.replace('_', "");
    i64::from_str_radix(&num_str, radix.numeric_radix())
        .map(|n| if negative { -n } else { n })
}

fn parse_expr(expression: Pairs<Rule>) -> Result<Expr> {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::number_hex | Rule::number_dec => parse_expr(primary.into_inner()),
            Rule::dec => parse_num(primary.as_str(), Radix::Dec)
                .with_context(|| format!("failed to parse decimal number: {}", primary.as_str()))
                .map(|num| Expr::Num(num, Radix::Dec)),
            Rule::dec_with_prefix => parse_num(primary.as_str(), Radix::DecWithPrefix)
                .with_context(|| format!("failed to parse decimal number with prefix: {}", primary.as_str()))
                .map(|num| Expr::Num(num, Radix::DecWithPrefix)),
            Rule::hex_with_prefix => parse_num(primary.as_str(), Radix::HexWithPrefix)
                .with_context(|| format!("failed to parse hex number with prefix: {}", primary.as_str()))
                .map(|num| Expr::Num(num, Radix::HexWithPrefix)),
            Rule::hex => parse_num(primary.as_str(), Radix::Hex)
                .with_context(|| format!("failed to parse hex number: {}", primary.as_str()))
                .map(|num| Expr::Num(num, Radix::Hex)),
            Rule::oct => parse_num(primary.as_str(), Radix::Oct)
                .with_context(|| format!("failed to parse octal number: {}", primary.as_str()))
                .map(|num| Expr::Num(num, Radix::Oct)),
            Rule::bin => parse_num(primary.as_str(), Radix::Bin)
                .with_context(|| format!("failed to parse binary number: {}", primary.as_str()))
                .map(|num| Expr::Num(num, Radix::Bin)),
            Rule::ans => Ok(Expr::Ans),
            Rule::expr_dec | Rule::expr_hex => parse_expr(primary.into_inner()),
            rule => unreachable!("parse_expr expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let lhs = lhs?;
            let rhs = rhs?;
            let op = match op.as_rule() {
                Rule::add => Op::Add,
                Rule::subtract => Op::Sub,
                Rule::multiply => Op::Mul,
                Rule::divide => Op::Div,
                Rule::rem => Op::Rem,
                Rule::and => Op::And,
                Rule::or => Op::Or,
                Rule::xor => Op::Xor,
                Rule::lshift => Op::LShift,
                Rule::rshift => Op::RShift,
                rule => unreachable!("expected operator rule, found {:?}", rule),
            };
            Ok(Expr::BinOp(BinOpExpr {
                left: Box::new(lhs),
                right: Box::new(rhs),
                op,
            }))
        })
        .parse(expression)
}

pub mod eval {
    use super::*;

    pub fn eval_expr(expr: &Expr, ans: i64) -> Result<i64, CorkError> {
        match &expr {
            Expr::Num(num, _) => Ok(*num),
            Expr::BinOp(expr) => {
                let left = eval_expr(expr.left.as_ref(), ans)?;
                let right = eval_expr(expr.right.as_ref(), ans)?;
                match expr.op {
                    // note that order does not matter here
                    Op::Add => Ok(left + right),
                    Op::Sub => Ok(left - right),
                    Op::Mul => Ok(left * right),
                    Op::And => Ok(left & right),
                    Op::Xor => Ok(left ^ right),
                    Op::Or => Ok(left | right),
                    Op::LShift => Ok(left << right),
                    Op::RShift => Ok(left >> right),
                    Op::Div => {
                        if right == 0 {
                            Err(CorkError::Eval(String::from("Cannot divide by 0")))
                        } else {
                            Ok(left / right)
                        }
                    }
                    Op::Rem => {
                        if right == 0 {
                            Err(CorkError::Eval(String::from("Cannot divide by 0")))
                        } else {
                            Ok(left % right)
                        }
                    }
                }
            }
            Expr::Ans => Ok(ans),
        }
    }
}
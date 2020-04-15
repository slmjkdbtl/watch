// wengwengweng

use pom::parser::*;
use pom::char_class::*;

enum Stat {
	Comment,
	Rule(Rule),
}

#[derive(Clone, Debug)]
pub struct Rule {
	pub pat: String,
	pub cmd: String,
}

fn pat<'a>() -> Parser<'a, u8, String> {
	return
		none_of(b":")
			.repeat(1..)
			.convert(String::from_utf8) - seq(b":")
		;
}

fn cmd<'a>() -> Parser<'a, u8, String> {
	return
		none_of(b"\n")
			.repeat(1..)
			.convert(String::from_utf8)
		;
}

fn rule<'a>() -> Parser<'a, u8, Stat> {
	return (pat() - blank() + cmd()).map(|(pat, cmd)| {
		return Stat::Rule(Rule {
			pat: pat,
			cmd: cmd,
		});
	});
}

fn blank<'a>() -> Parser<'a, u8, ()> {
	return
		is_a(multispace)
			.repeat(0..)
			.discard()
		;
}

fn comment<'a>() -> Parser<'a, u8, Stat> {
	return
		sym(b'#')
			* none_of(b"\n\r")
			.repeat(0..)
			.discard()
			.map(|_| Stat::Comment)
			;
}

fn all<'a>() -> Parser<'a, u8, Vec<Stat>> {
	return
		blank()
		* list(comment() | rule(), blank())
		- blank();
}

pub fn parse_watchfile(code: &str) -> Result<Vec<Rule>, pom::Error> {
	return all().parse(code.as_bytes()).map(|stats| {
		return stats.into_iter().map(|s| {
			return match s {
				Stat::Rule(r) => Some(r),
				_ => None,
			};
		}).flatten().collect();
	});
}


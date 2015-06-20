use ::JsResult;
use rt::{JsEnv, JsArgs, JsValue, JsFnMode};
use gc::*;
use syntax::reader::StringReader;
use self::writer::JsonWriter;

mod lexer;
mod parser;
mod writer;

// 15.12.2 parse ( text [ , reviver ] )
pub fn JSON_parse(env: &mut JsEnv, _mode: JsFnMode, args: JsArgs) -> JsResult<Local<JsValue>> {
	let string = try!(args.arg(env, 0).to_string(env)).to_string();
	
	let mut reader = StringReader::new("(json)", &string);
	let mut lexer = try!(lexer::Lexer::new(&mut reader));
	
	let result = try!(parser::Parser::parse_json(env, &mut lexer));
	
	Ok(result)
}

// 15.12.3 stringify ( value [ , replacer [ , space ] ] )
pub fn JSON_stringify(env: &mut JsEnv, mode: JsFnMode, args: JsArgs) -> JsResult<Local<JsValue>> {
	try!(JsonWriter::new(env, mode, args)).write()
}

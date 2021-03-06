use cfgrammar::yacc::{YaccKind};
use lrlex::LexerBuilder;
use lrpar::CTParserBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lex_rule_ids_map = CTParserBuilder::new()
        .yacckind(YaccKind::Grmtools)
        .visibility(lrpar::Visibility::Public)
        .process_file_in_src("fort500.y")?;
    LexerBuilder::new()
        .rule_ids_map(lex_rule_ids_map)
        .process_file_in_src("fort500.l")?;
    Ok(())
}
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "f.pest"]
pub struct Grammar;
fn main() -> anyhow::Result<()> {
    let unparsed_file = Grammar::parse(Rule::record, "-111,111,2222")?;
    println!("{:?}", unparsed_file);
    Ok(())
}

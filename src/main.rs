use miette::Result;

fn main() -> Result<()> {
    let tokens = easl::scan()?;

    println!(
        "{:?}",
        tokens
            .clone()
            .map(|t| t.token_type)
            .collect::<Vec<easl::TokenType>>()
    );

    let expr = easl::parse(tokens)?;

    println!("{:?}", expr);

    Ok(())
}

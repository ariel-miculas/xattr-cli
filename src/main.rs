use std::env;

fn print_xattr(file: &str) -> anyhow::Result<()> {
    let mut xattrs = xattr::list(file)?.peekable();

    if xattrs.peek().is_none() {
        println!("no xattr set");
        return Ok(());
    }

    println!("Extended attributes of {file}:");
    for attr in xattrs {
        println!(
            "{} -> {}",
            attr.to_string_lossy(),
            xattr::get(file, &attr)?
                .as_deref()
                .map(std::str::from_utf8)
                .transpose()?
                .unwrap_or("<unknown>")
        );
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter().skip(1) {
        print_xattr(&arg)?;
    }

    Ok(())
}

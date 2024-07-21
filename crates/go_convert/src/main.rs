use go_convert::GoConversion;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args_os().collect::<Vec<_>>();

    if args.len() > 1 {
        let go_pkgs = gosyn::parse_dir(&args[1])?;
        // let value = serde_json::to_value(&go_pkgs)?;
        // println!("{}", serde_json::to_string_pretty(&value)?);

        let bump = bumpalo::Bump::new();
        let convert = GoConversion::new(&bump);
        let pkgs = go_pkgs
            .iter()
            .map(|(name, pkg)| (name.clone(), convert.convert(pkg)))
            .collect::<HashMap<_, _>>();
        println!("{pkgs:#?}");

        Ok(())
    } else {
        eprintln!("Usage: convert <input>");
        std::process::exit(1);
    }
}

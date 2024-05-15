use clap::Parser;
use rcli::{process_csv, process_genpass, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    //println!("{opts:?}");

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, &output, opts.format)?
        }
        SubCommand::GenPass(opts) => {
            if opts.length < 4 {
                println!("password too short");
            } else if opts.no_uppercase && opts.no_lowercase && opts.no_number && opts.no_symbol {
                println!("invalid options");
            } else {
                process_genpass(
                    opts.length,
                    opts.no_uppercase,
                    opts.no_lowercase,
                    opts.no_number,
                    opts.no_symbol,
                )?
            }
        }
    }

    Ok(())
}

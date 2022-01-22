use anyhow::Result;
use clap::Parser;
use serde_json::Value;
use std::path::PathBuf;
use tera::{Context, Tera};

#[derive(Parser, Debug)]
#[clap()]
struct Args {
    name: String,

    #[clap(short, long, default_value = "context.toml")]
    context: Vec<PathBuf>,

    #[clap(short, long, default_value = "templates")]
    dir: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let tera = Tera::new(&format!("{}/**/*", &args.dir.to_str().unwrap()))?;

    let mut all_context = Value::Null;
    for path in args.context {
        let context = std::fs::read_to_string(&path).unwrap_or_else(|_| "".to_string());
        let extension = path.extension().map(|s| s.to_str().unwrap().clone());

        let context: Value = match extension {
            Some("toml") => {
                let context = context.parse::<toml::Value>()?;
                let context = serde_json::to_string(&context)?;
                serde_json::from_str(&format!("{context}"))?
            }
            Some("json") => serde_json::from_str(&context)?,
            _ => unimplemented!(),
        };
        merge(&mut all_context, &context);
    }

    let res = tera.render(&args.name, &Context::from_serialize(&all_context)?)?;
    print!("{res}");

    Ok(())
}

// Source: https://stackoverflow.com/questions/47070876/how-can-i-merge-two-json-objects-with-rust
fn merge(a: &mut Value, b: &Value) {
    if let Value::Object(a) = a {
        if let Value::Object(b) = b {
            for (k, v) in b {
                if v.is_null() {
                    a.remove(k);
                } else {
                    merge(a.entry(k).or_insert(Value::Null), v);
                }
            }

            return;
        }
    }

    *a = b.to_owned();
}

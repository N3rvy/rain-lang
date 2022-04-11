use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the main module
    main: String,

    /// Path and name of the definition module (ex. engine.d.vrs:engine)
    #[clap(long)]
    def: Option<String>,

    // Path where to output the build result
    #[clap(long, default_value_t = String::from("output.wasm"))]
    out: String,
}

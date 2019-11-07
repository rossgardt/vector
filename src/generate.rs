use crate::topology::config::{SinkDescription, SourceDescription, TransformDescription};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub struct Opts {
    /// Generate expression, e.g. stdin|json_parser,add_fields|console
    ///
    /// Three comma-separated lists of sources, transforms and sinks, separated
    /// by pipes. If subsequent component types are not needed then their pipes
    /// can be omitted from the expression.
    ///
    /// For example:
    ///
    /// `|json_parser` prints a `json_parser` transform.
    ///
    /// `||file,http` prints a `file` and `http` sink.
    ///
    /// `stdin||http` prints a `stdin` source and an `http` sink.
    ///
    /// Vector makes a best attempt at constructing a sensible topology. The
    /// first transform generated will consume from all sources and subsequent
    /// transforms will consume from their predecessor. All sinks will consume
    /// from the last transform or, if none are specified, from all sources. It
    /// is then up to you to restructure the `inputs` of each component to build
    /// the topology you need.
    ///
    /// Generated components are given incremental names (`source1`, `source2`,
    /// etc) which should be replaced in order to provide better context.
    expression: String,
}

pub fn cmd(opts: &Opts) -> exitcode::ExitCode {
    println!("{}", opts.expression);
    exitcode::OK
}

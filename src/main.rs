mod chart;
mod cli;
mod csv_writer;
mod distribution;
mod rainfall;
mod types;
mod validator;

use std::process;

use clap::Parser;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err:#}");
        process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let validated = validator::validate(&cli)?;

    let increments = rainfall::calculate(&validated.rainfall_params);
    let data = distribution::arrange(&increments, validated.pattern, validated.rainfall_params.t);

    let output_path = &validated.output_config.output_path;

    match validated.output_config.format {
        types::OutputFormat::Png => {
            chart::render(&data, output_path, validated.rainfall_params.t)?;
            println!("PNG output: {}", output_path.display());
        }
        types::OutputFormat::Csv => {
            let csv_path = output_path.with_extension("csv");
            csv_writer::write(&data, &csv_path)?;
            println!("CSV output: {}", csv_path.display());
        }
        types::OutputFormat::Both => {
            chart::render(&data, output_path, validated.rainfall_params.t)?;
            println!("PNG output: {}", output_path.display());

            let csv_path = output_path.with_extension("csv");
            csv_writer::write(&data, &csv_path)?;
            println!("CSV output: {}", csv_path.display());
        }
    }

    Ok(())
}

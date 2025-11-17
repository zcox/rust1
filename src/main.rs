//! Temperature converter CLI application
//!
//! This application demonstrates:
//! - Using library functions
//! - Command-line argument parsing with clap
//! - Pattern matching for error handling
//! - Accessing build-time environment variables

use clap::{Parser, ValueEnum};
use rust1::{celsius_to_fahrenheit, fahrenheit_to_celsius};

/// Simple temperature converter between Celsius and Fahrenheit
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Type of conversion to perform
    #[arg(value_enum)]
    conversion_type: ConversionType,

    /// Temperature value to convert
    temperature: f64,

    /// Show build information
    #[arg(short, long)]
    build_info: bool,
}

#[derive(Debug, Clone, ValueEnum)]
enum ConversionType {
    /// Convert from Celsius to Fahrenheit
    Celsius,
    /// Convert from Fahrenheit to Celsius
    Fahrenheit,
}

fn main() {
    let args = Args::parse();

    // Show build information if requested
    if args.build_info {
        println!("Build timestamp: {}", env!("BUILD_TIMESTAMP"));
        println!("Build profile: {}", env!("BUILD_PROFILE"));
        println!();
    }

    // Perform the conversion based on the type
    let result = match args.conversion_type {
        ConversionType::Celsius => match celsius_to_fahrenheit(args.temperature) {
            Ok(fahrenheit) => {
                println!("{}°C = {:.2}°F", args.temperature, fahrenheit);
                Ok(())
            }
            Err(e) => {
                eprintln!("Error: Temperature is below absolute zero (-273.15°C)");
                eprintln!("Details: {:?}", e);
                Err(())
            }
        },
        ConversionType::Fahrenheit => match fahrenheit_to_celsius(args.temperature) {
            Ok(celsius) => {
                println!("{}°F = {:.2}°C", args.temperature, celsius);
                Ok(())
            }
            Err(e) => {
                eprintln!("Error: Temperature is below absolute zero (-459.67°F)");
                eprintln!("Details: {:?}", e);
                Err(())
            }
        },
    };

    // Exit with error code if conversion failed
    if result.is_err() {
        std::process::exit(1);
    }
}

use std::{fmt::Display, process::exit};

#[derive(Debug, Copy, Clone)]
enum TemperatureScale {
    Celsius(),
    Fahrenheit(),
    Kelvin(),
}

impl From<String> for TemperatureScale {
    fn from(scale: String) -> Self {
        match scale.to_uppercase().as_str() {
            "C" => TemperatureScale::Celsius(),
            "F" => TemperatureScale::Fahrenheit(),
            "K" => TemperatureScale::Kelvin(),
            _ => panic!("Invalid temperature scale"),
        }
    }
}

impl Display for TemperatureScale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TemperatureScale::Celsius() => write!(f, "Celsius"),
            TemperatureScale::Fahrenheit() => write!(f, "Fahrenheit"),
            TemperatureScale::Kelvin() => write!(f, "Kelvin"),
        }
    }
}

struct Temprature {
    value: f64,
    scale: TemperatureScale,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let temp: &String = &args[1];
    let from: TemperatureScale = TemperatureScale::from((args[2]).clone());
    let to: TemperatureScale = TemperatureScale::from((args[3]).clone());
    let from_struct = Temprature {
        value: temp.parse().unwrap(),
        scale: from,
    };

    let new_temp: f64;

    match from {
        TemperatureScale::Celsius() => match to {
            TemperatureScale::Fahrenheit() => {
                new_temp = c_to_f(temp.parse().unwrap());
            }
            TemperatureScale::Kelvin() => {
                new_temp = c_to_k(temp.parse().unwrap());
            }
            _ => {
                println!("Invalid conversion");
                exit(1)
            }
        },
        TemperatureScale::Fahrenheit() => match to {
            TemperatureScale::Celsius() => {
                new_temp = f_to_c(temp.parse().unwrap());
            }
            TemperatureScale::Kelvin() => {
                new_temp = f_to_k(temp.parse().unwrap());
            }
            _ => {
                println!("Invalid conversion");
                exit(1)
            }
        },
        TemperatureScale::Kelvin() => match to {
            TemperatureScale::Celsius() => {
                new_temp = k_to_c(temp.parse().unwrap());
            }
            TemperatureScale::Fahrenheit() => {
                new_temp = k_to_f(temp.parse().unwrap());
            }
            _ => {
                println!("Invalid conversion");
                exit(1)
            }
        },
    }

    print_temp(
        from_struct,
        Temprature {
            value: (new_temp),
            scale: (to),
        },
    )
}

fn print_temp(from: Temprature, to: Temprature) {
    println!(
        "{} degrees in {} is {} in {}",
        from.value, from.scale, to.value, to.scale
    );
}

fn c_to_f(temp: f64) -> f64 {
    (temp * 9.0 / 5.0) + 32.0
}

fn c_to_k(temp: f64) -> f64 {
    temp + 273.15
}

fn f_to_c(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}

fn f_to_k(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0 + 273.15
}

fn k_to_c(temp: f64) -> f64 {
    temp - 273.15
}

fn k_to_f(temp: f64) -> f64 {
    (temp - 273.15) * 9.0 / 5.0 + 32.0
}

use rand::distributions::Standard;
use rand::Rng;
use std::fs::File;
use std::io::Write;

fn inverse_transform_sampling() {
    let lambda = 0.01;
    let size = 10000;
    let mut rng = rand::thread_rng();

    let mut exponential_values: Vec<f64> = Vec::with_capacity(size);

    for _ in 0..size {
        let u: f64 = rng.sample(Standard);
        let exp_value = -1.0 / lambda * (1.0 - u).ln();
        exponential_values.push(exp_value);
    }

    // Imprimer les premières valeurs pour vérification
    for value in exponential_values.iter().take(10) {
        println!("{}", value);
    }

    // Enregistrement des valeurs dans un fichier pour analyse

    let mut file = File::create("exponential_values.csv").unwrap();
    for value in exponential_values {
        writeln!(file, "{}", value).unwrap();
    }
}

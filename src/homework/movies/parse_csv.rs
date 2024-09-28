use crate::homework::movies::movies::Movie;
use chrono::{DateTime, Local};
use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

fn parse_row(row: &str) -> Result<Movie, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false) // Desactivar encabezados si el CSV no tiene encabezados.
        .from_reader(row.as_bytes());

    for result in rdr.records() {
        let record = result?;
        let movie: Movie = record.deserialize(None)?;
        return Ok(movie);
    }
    Err("No se pudo parsear la fila".into())
}

fn parse_csv(file_path: &str) -> Result<Vec<Movie>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_reader(File::open(file_path)?);
    let mut movies = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let row = record.iter().collect::<Vec<_>>().join(",");
        let movie = parse_row(&row)?;
        movies.push(movie);
    }

    Ok(movies)
}

// Ejemplo de uso
fn main() -> Result<(), Box<dyn Error>> {
    let movies = parse_csv("ruta/al/archivo.csv")?;
    for movie in movies {
        println!("{:?}", movie);
    }
    Ok(())
}

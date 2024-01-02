use csv::ReaderBuilder;
use plotters::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "src/seu_arquivo.csv"; // Se o arquivo CSV está no diretório 'src'
 // Altere para o caminho do seu arquivo CSV
    let root = BitMapBackend::new("grafico.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let mut data: Vec<(f64, f64)> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let x: f64 = record[0].parse()?;
        let y: f64 = record[1].parse()?;
        data.push((x, y));
    }

    let x_range = 0.0..10.0; // Ajuste de acordo com seus dados
    let y_range = 0.0..100.0; // Ajuste de acordo com seus dados

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_range, y_range)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(data, &RED))?;

    Ok(())
}

use super::{GITHUB_PREFIX, SUMMARY};
use plotters::prelude::*;

const IMG_WIDTH: usize = 800;
const IMG_HEIGHT: usize = 600;

/// (Path, URL)
pub fn generate_image_file(image_name: String) -> (String, String) {
	if SUMMARY.clone().is_some() {
		// In Github Actions
		let image_path = format!("test/{}", image_name);
		let image_url = format!("{}/{}", GITHUB_PREFIX, image_name);

		return (image_path, image_url);
	}

	// In Local Test
	let image_path = format!("test/{}", image_name);
	let image_url = format!("./{}", image_name);
	return (image_path, image_url);
}

pub fn generate_line_chart(filepath: &str, title: &str, label: &str, data: &[u128]) {
	println!("Write line chart to {}", filepath);
	let root = BitMapBackend::new(
		filepath,
		(IMG_WIDTH.try_into().unwrap(), IMG_HEIGHT.try_into().unwrap())
	).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let max_x = data.len();
    let max_y = *data.iter().max().unwrap_or(&1);

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..max_x, 0u128..max_y)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            data.iter().enumerate().map(|(x, y)| (x, *y)),
            &RED,
        ))
        .unwrap()
        .label(label)
        .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &RED));

    chart.configure_series_labels().draw().unwrap();
    root.present().unwrap();
}

pub fn generate_dot_series_chart(filepath: &str, title: &str, x_label: &str, y_label: &str, data: &[u128]) {
	let root = BitMapBackend::new(
		filepath,
		(IMG_WIDTH.try_into().unwrap(), IMG_HEIGHT.try_into().unwrap())
	).into_drawing_area();
	root.fill(&WHITE).unwrap();
	let y_max = data.iter().cloned().max().unwrap_or(0);
	let mut chart = ChartBuilder::on(&root)
		.margin(20)
		.caption(title, ("Consolas", 30))
		.x_label_area_size(40)
		.y_label_area_size(40)
		.build_cartesian_2d(0..data.len(), 0u128..(y_max + 50))
		.unwrap();

	chart.configure_mesh()
		.x_desc(x_label)
		.y_desc(y_label)
		.draw()
		.unwrap();

	chart.draw_series(
        data.iter().enumerate().map(|(x, y)| {
            Circle::new((x, *y), 2, RED.filled())
        })
    ).unwrap();
}
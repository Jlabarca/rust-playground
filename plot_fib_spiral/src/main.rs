use plotters::prelude::*;
extern crate itertools_num;

use itertools_num::linspace;
/**
 * Using plotter to find a proper way to generate positions arround a center in a circular way.
 */
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("result.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE);
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(-20f32..20f32, -20f32..20f32)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        // .x_labels(5)
        // .y_labels(5)
        // // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;


    let mut dots = Vec::new();

    // Center position


    add_dots((-12.0, 12.0), 5, &mut dots);
    add_dots((-12.0, -12.0), 12, &mut dots);
    add_dots((12.0, 12.0), 56, &mut dots);
    add_dots((12.0, -12.0), 100, &mut dots);
    add_dots((0.0, 0.0), 1000, &mut dots);

    for dot in dots.iter().cloned() {
        println!("({}, {})", dot.0, dot.1);
    }

    //And we can draw something in the drawing area
    // chart.draw_series(LineSeries::new(
    //     dots.iter().cloned(),
    //     &RED,
    // ))?;
    //Similarly, we can draw point series
    chart.draw_series(PointSeries::of_element(
        dots.iter().cloned(),
        2,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
            //+ Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
        },
    ))?;
    root.present()?;
    Ok(())
}

fn add_dots(position: (f32, f32), total: usize, dots: &mut Vec<(f32, f32)>)  {

    let pi = std::f32::consts::PI;
    let desface = 22.0f32;
    let phi = ( 1.0 + desface.sqrt() ) / 2.0;
	let totalf32 = total as f32;
    let grow_ratio = 1.0;
    let growSizeRelation = 0.2;
    let mut growReductionBySize = growSizeRelation * totalf32;
    // growReductionBySize = if growReductionBySize > grow_ratio {
    //     0.1
    // } else {
    //     growReductionBySize
    // };

    let growRatio = totalf32 / ( 10. + growReductionBySize);

    for i in 0..total {

        let if32 = ( -(total as i16) as f32 + 1. + 2. * i as  f32) as f32;

		let theta = 2.0 * pi * if32 / phi;
        let	cphi = growRatio * ( ( totalf32 + if32 ) * ( totalf32 - if32 ) ).sqrt() / totalf32;
        let dot = (position.0 + cphi * theta.sin(), position.1 + cphi * theta.cos());

        //println!("({}, {})", dot.0, dot.1);
        dots.push(dot);
    }
}


// inline std::vector< Vector3Type > sphere_fibonacci_points ( int n = 100 )
// {
// 	double cphi;
// 	double i_r8,n_r8;
// 	double phi,sphi,theta;
// 	const double pi = 3.141592653589793;

// 	phi = ( 1.0 + std::sqrt ( 5.0 ) ) / 2.0;
// 	n_r8 = ( double ) ( n );

// 	std::vector< Vector3Type > points;

// 	for (int j = 0; j < n; j++ )
// 	{
// 		i_r8 = ( double ) ( - n + 1 + 2 * j );
// 		theta = 2.0 * pi * i_r8 / phi;
// 		sphi = i_r8 / n_r8;
// 		cphi = std::sqrt ( ( n_r8 + i_r8 ) * ( n_r8 - i_r8 ) ) / n_r8;

// 		points.push_back( Vector3Type(cphi * std::sin ( theta ), cphi * std::cos ( theta ), sphi) );
// 	}

// 	return points;
// }
/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    Vec::new()
}
/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0;count]
}
/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci(count: usize) -> Vec<u8> {
    let mut result = create_buffer(count);
    (result[0], result[1]) = (1,1);

    for idx in 2..result.len() {
        result[idx] = result[idx - 1] + result[idx - 2]
    };
    result
}
// use num_traits::Float;
// pub fn linspace<T: Float + TryFrom<usize>>(x0: T, xend: T, n: usize) -> Vec<T> {
//     let to_float = |i: usize| i.try_into().unwrap_or_else(|_| panic!());
//     let dx = (xend - x0) / to_float(n - 1);
//     (0..n).map(|i| x0 + to_float(i) * dx).collect()
// }

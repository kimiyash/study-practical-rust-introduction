
#[derive(Default)]
pub struct Polygon<T> {
    pub vertexes: Vec<T>,
    pub stroke_width: u32,
}

trait Coordinates {}

#[derive(Default)]
struct CartesianCoord {
    x: f64,
    y: f64,
}
impl Coordinates for CartesianCoord {}

#[derive(Default)]
struct PolarCoord {
    r: f64,
    theta: f64,
}
impl Coordinates for PolarCoord {}

fn main() {
    let vertexes = vec![
        CartesianCoord {x: 0.0, y: 0.0},
        CartesianCoord {x: 50.0, y: 0.0},
        CartesianCoord {x: 30.0, y: 20.0},
    ];
    let poly = Polygon { vertexes, .. Default::default() };

    let vertexes = vec![
        PolarCoord {r: 0.0, theta: 0.0},
        PolarCoord {r: 50.0, theta: 0.0},
        PolarCoord {r: 30.0, theta: 20.0},
    ];
    let poly = Polygon { vertexes, .. Default::default() };
}

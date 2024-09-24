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

#[derive(Default)]
struct A {
    f0: u8,
    f1: u32,
    f2: u8,
}

fn main() {
    let vertexes = vec![
        CartesianCoord { x: 0.0, y: 0.0 },
        CartesianCoord { x: 50.0, y: 0.0 },
        CartesianCoord { x: 30.0, y: 20.0 },
    ];
    let poly = Polygon {
        vertexes,
        ..Default::default()
    };

    let vertexes = vec![
        PolarCoord { r: 0.0, theta: 0.0 },
        PolarCoord {
            r: 50.0,
            theta: 0.0,
        },
        PolarCoord {
            r: 30.0,
            theta: 20.0,
        },
    ];
    let poly = Polygon {
        vertexes,
        ..Default::default()
    };

    let a: A = Default::default();
    println!(
        "struct A ({} bytes)\n\tf0: {:p}\n\tf1: {:p}\n\tf2: {:p}",
        std::mem::size_of::<A>(),
        &a.f0,
        &a.f1,
        &a.f2
    );
}

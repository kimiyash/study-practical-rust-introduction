use std::f64::consts::PI;

#[derive(Debug, Clone)]
struct CartesianCoord {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
struct PolarCoord {
    r: f64,
    theta: f64,
}

trait Coordinates {
    fn to_cartesian(self) -> CartesianCoord;
    fn from_cartesian(cart: CartesianCoord) -> Self;
}

impl Coordinates for CartesianCoord {
    fn to_cartesian(self) -> CartesianCoord {
        self
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        cart
    }
}

impl Coordinates for PolarCoord {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.r * self.theta.cos(),
            y: self.r * self.theta.sin(),
        }
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        PolarCoord {
            r: (cart.x * cart.x + cart.y * cart.y).sqrt(),
            theta: (cart.y / cart.x).atan(),
        }
    }
}

impl Coordinates for (f64, f64) {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.0,
            y: self.1,
        }
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        (cart.x, cart.y)
    }
}

fn print_point(point: impl Coordinates) {
    let p = point.to_cartesian();
    println!("({}, {})", p.x, p.y);
}

fn as_cartesian<P: Coordinates + Clone>(point: &P) -> CartesianCoord {
    point.clone().to_cartesian()
}

fn double_point<P: Coordinates>(point: P) -> P {
    let mut cart = point.to_cartesian();
    cart.x *= 2.0;
    cart.y *= 2.0;
    P::from_cartesian(cart)
}

fn make_point<T>(x: T, y: T) -> CartesianCoord
where
    (T, T): Coordinates,
{
    (x, y).to_cartesian()
}

trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

fn to<T>(i: i32) -> T
where
    i32: ConvertTo<T>,
{
    i.convert()
}

struct Matrix([[f64; 2]; 2]);
trait LinearTrasform: Coordinates {
    fn transform(self, matrix: &Matrix) -> Self
    where
        Self: Sized,
    {
        let mut cart = self.to_cartesian();
        let x = cart.x;
        let y = cart.y;
        let m = matrix.0;

        cart.x = m[0][0] * x + m[0][1] * y;
        cart.y = m[1][0] * x + m[1][1] * y;
        Self::from_cartesian(cart)
    }

    fn rotate(self, theta: f64) -> Self
    where
        Self: Sized,
    {
        self.transform(&Matrix([
            [theta.cos(), -theta.sin()],
            [theta.sin(), theta.cos()],
        ]))
    }

    fn rotate_matrix(theta: f64) -> Matrix {
        Matrix([[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]])
    }
}

impl LinearTrasform for CartesianCoord {}

impl LinearTrasform for PolarCoord {
    fn rotate(mut self, theta: f64) -> Self {
        self.theta += theta;
        self
    }
}

trait Dimension {
    const DIMENSION: u32;
}

impl Dimension for CartesianCoord {
    const DIMENSION: u32 = 2;
}

fn main() {
    let point = (1.0, 1.0);

    let c = point.to_cartesian();
    println!("{:?}", c);

    let p = PolarCoord::from_cartesian(c);
    println!("{:?}", p);

    print_point((0.0, 1.0));
    print_point(PolarCoord {
        r: 1.0,
        theta: std::f64::consts::PI / 2.0,
    });

    let d = as_cartesian(&p);
    let _d = double_point(d);
    let _ = make_point(1.0, 2.0);

    let xy = (2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0);
    let c = xy.to_cartesian();
    let c = c.rotate(PI / 4.0);
    println!("{:?}", c);
    let c = c.rotate(-PI / 2.0);
    println!("{:?}", c);
    let c = c.rotate(PI / 4.0);
    println!("{:?}", c);

    let r = PolarCoord::from_cartesian(xy.to_cartesian());
    println!("{:?}", r);
    let r = r.transform(&CartesianCoord::rotate_matrix(PI / 4.0));
    println!("{:?}", r);
    let r = r.transform(&CartesianCoord::rotate_matrix(-PI / 2.0));
    println!("{:?}", r);
    let r = r.transform(&CartesianCoord::rotate_matrix(PI / 4.0));
    println!("{:?}", r);

    let dim = CartesianCoord::DIMENSION;
    const DIM: u32 = CartesianCoord::DIMENSION;
}

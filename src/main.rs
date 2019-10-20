use num::complex::Complex;

type Point = Complex<f64>;



fn angle(pa: Point, pb: Point, pc: Point) -> f64
{
    ((pa-pb)/(pc-pb)).arg()
}

fn triangle_area(pa: Point, pb: Point, pc: Point) -> f64
{
    let dist_ab = (pb-pa).norm();
    let dist_bc = (pb-pc).norm();
    dist_ab*dist_bc*angle(pa,pb,pc).sin()/2.
}


struct Polygon
{
    points: Vec<Point>
}


impl Polygon
{
    fn new(points: Vec<(f64, f64)>) -> Self
    {
        Self {
            points: points.iter().map(|&(re, co)| Complex::new(re, co)).collect()
        }
    }

    fn area(&self) -> f64
    {
        if self.points.len() < 3
        {
            0.
        }
        else
        {   
            let first_point = self.points[0];
            let mid_iter = self.points.iter().skip(1);
            let last_iter = self.points.iter().skip(2);
            mid_iter.zip(last_iter).fold(0., |area, (&mid_point, &last_point)|
                                         {
                                             println!("partial area: {}", area);
                                             area + triangle_area(first_point, mid_point, last_point)
                                         }
            ).abs()
        }
        
    }
    
    
}


fn main() {

    let a = Complex::new(0., 8.);
    let b = Complex::new(0., 0.);
    let c = Complex::new(1., 0.);

    let poly = Polygon::new(vec![
        (0., 0.),
        (0., 5.),
        (3., 5.),
        (3., 2.),
        (3., 3.),
        (5., 3.),
        (5., 5.),
        (5., 0.),
    ]);

    
    println!("angle: {} pi", angle(a,b,c)/std::f64::consts::PI);
    println!("area: {} ", triangle_area(a,b,c));

    println!("poly area {}", poly.area());
    
}

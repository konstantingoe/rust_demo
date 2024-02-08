use rand::prelude::*;
use rand_distr::{Distribution, StandardNormal};

pub struct LinearRegression {
    pub slope: f64,
    pub intercept: f64,
}

fn main() {
    // Create a new instance of the LinearRegression struct
    let mut linear_regression_model =LinearRegression::new();
    println!("Slope: {}", linear_regression_model.slope);

    //create toy data:
    let mut rng = thread_rng();

    let x: Vec<f64> = StandardNormal.sample_iter(&mut rng).take(100).collect();
    let epsilon: Vec<f64> = StandardNormal.sample_iter(&mut rng).take(100).collect();
    let true_b1 = 2.5;
    let true_b0 = 2.0;
    let y: Vec<f64> = x.iter().map(|x| &true_b1 * x + &true_b0).collect::<Vec<f64>>();
    let y: Vec<f64> = y.iter().zip(epsilon.iter()).map(|(x, e)| x + e).collect();

    println!("y: {:?}", y);

    linear_regression_model.fit(x, y);
    println!("The slope is {}",linear_regression_model.slope);
    println!("The intercept is {}",linear_regression_model.intercept);

}

impl LinearRegression{
    // make new() method available
    pub fn new() -> LinearRegression {
        LinearRegression {
            slope: 0.0,
            intercept: 0.0,
        }
    }
    // we will add the fit method here
    pub fn fit (&mut self,x:Vec<f64>,y:Vec<f64>) {
        // exit if the sample size is different
        if x.len()!=y.len(){
            panic!("The number of input and output values is different");
        }
        let n=x.len();
        // iterators are evaluated lazily, so we need to call sum() to get the actual value
        let sum_x:f64=x.iter().sum::<f64>();
        let sum_y:f64=y.iter().sum::<f64>();

        let sum_of_x_y=x.iter().zip(y.iter()).map(|(&x, &y)| x * y).sum::<f64>();
        let square_sum_x=x.iter().map(|&x|x*x).sum::<f64>();

        //get beta_1 and beta_0
        self.slope=(n as f64*sum_of_x_y-sum_x*sum_y)/(n as f64*square_sum_x-sum_x*sum_x);

        self.intercept=(sum_y*square_sum_x-sum_x*sum_of_x_y)/(n as f64*square_sum_x-sum_x*sum_x);
    }
}
pub fn fast_eval(text: &str) -> f64
{
    use fasteval::{EmptyNamespace, ez_eval};

    let mut ns = EmptyNamespace;
    let val = ez_eval(text, &mut ns).unwrap();
    //println!("fast_eval: {:?}", val);
    return val;
}

pub fn eval_expr(text: &str) -> f64
{
    use evalexpr::*;
    let val = eval(text).unwrap();
    //println!("evalexpr: {:?}", val);
    let x = val.as_number().unwrap();
    x as f64
}
//use evals_tests::{fast_eval, eval_expr};

fn main() 
{
    let a: u32 = 0b11001111;
    let b: u32 = 0b10000000;

    let bval = a & b;
    println!("a: {}, b: {}, bval: {:?}", a, b, bval);

    //fast_eval("15&&4");
    //eval_expr("bitand(15, 4)");

    use exmex::{BinOp, Operator};

    let ops = [
        Operator {
            //bitand
            repr: "&",
            bin_op: Some(BinOp {
                apply: |a: u32, b: u32| { 
                    let x = a & b;
                    x  
                },
                prio: 0,
            }),
            unary_op: None,
        },
        Operator {
            //bitor
            repr: "|",
            bin_op: Some(BinOp {
                apply: |a: u32, b: u32| { 
                    let x = a | b;
                    x  
                },
                prio: 0,
            }),
            unary_op: None,
        },
    ];

    let expr = exmex::parse::<u32>("a&b", &ops).unwrap();
    print!("expr: {:?}", expr);

    let r = expr.eval(&[207, 128]).unwrap();
    print!("r: {:?}", r);

    // let expr = exmex::parse_with_default_ops::<f64>("207&128").unwrap();
    // let r = expr.eval(&[207.0, 128.0]).unwrap();

}    


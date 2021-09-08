use criterion::{criterion_group, criterion_main, Criterion};

use exmex::{BinOp, Operator};
use exmex::FlatEx;

const OPS: [Operator<u32>; 2] = [
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

fn ex_bitand_benchmark(c: &mut Criterion) 
{
   c.bench_function("classic", |b| b.iter(|| { 128&4 }));

   let expr: FlatEx<u32> = exmex::parse::<u32>("a&b", &OPS).unwrap();

   c.bench_function("exmex", |b| b.iter(|| { 
      let _r = expr.eval(&[207, 128]).unwrap();
    }));

   use evals_tests::{fast_eval, eval_expr};

   c.bench_function("fast_eval", |b| b.iter(|| { fast_eval("128&&4") }));
   c.bench_function("eval_expr", |b| b.iter(|| { eval_expr("bitand(128, 4)") }));

}

criterion_group!(benches, ex_bitand_benchmark);
criterion_main!(benches);
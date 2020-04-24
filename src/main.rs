//use rand::prelude::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;


mod op {

    #[macro_export]
    macro_rules! add_op {
	($table:tt, $func_name:ident) => {
	    $table.insert(stringify!($func_name).to_string(), $func_name);
	};
    }

    macro_rules! take_one {
	($s:ident, $v:ident, $ex:expr) => {
            match $s.pop() {
		Some($v) => {
		    $ex;
		}
		_ => {
                    println!("Error: stack underflow");
		}
            }
	}
    }

    macro_rules! take_two {
	($s:ident, $v1:ident, $v2:ident, $ex:expr) => {
            match $s.pop() {
		Some($v2) => {
                    match $s.pop() {
			Some($v1) => {
			    $ex;
			}
			_ => {
                            println!("Error: stack underflow");
			}
                    }
		}
		_ => {
                    println!("Error: stack underflow");
		}
            }
	}
    }

    pub fn dup(stack: &mut Vec<f32>) -> () {
	take_one!(stack, v, {
            stack.push(v);
            stack.push(v);
        });
    }

    pub fn exch(stack: &mut Vec<f32>) -> () {
	take_two!(stack, v1, v2, {
            stack.push(v2);
            stack.push(v1);
        });
    }

    pub fn pop(stack: &mut Vec<f32>) -> () {
	take_one!(stack, _v, {} );
    }

    pub fn copy(stack: &mut Vec<f32>) -> () {
	take_one!(stack, v, {
            let n = v as usize;
            if stack.len() >= n {
                let start = stack.len() - n;
                let end = stack.len();
                for i in start..end {
                    stack.push(stack[i]);
                }
            }
	});
    }

    pub fn roll(stack: &mut Vec<f32>) -> () {
	take_two!(stack, v1, v2, {
            let n = v1 as usize;
            let i = v2 as i32;
            if stack.len() >= n {
                let mut temp = Vec::new();
                let start = stack.len() - n;
                let end = stack.len();
                for _j in start..end {
                    temp.push(stack.pop().unwrap());
                }
                for j in (0..n).rev() {
                    let k = (n as i32 + j as i32 + i) as usize;
                    stack.push(temp[k%n]);
                }
            } else {
                println!("Error: stack underflow");
            }
	});
    }

    pub fn index(stack: &mut Vec<f32>) -> () {
	take_one!(stack, v, {
	    let n = v as usize;
            if stack.len() >= n {
                let i = stack.len() - n - 1;
                stack.push(stack[i]);
            }
	});
    }

    pub fn clear(stack: &mut Vec<f32>) -> () {
        stack.clear();
    }

    pub fn count(stack: &mut Vec<f32>) -> () {
        stack.push(stack.len() as f32);
    }

    pub fn add(stack: &mut Vec<f32>) -> () {
	take_two!(stack, v1, v2, stack.push(v1 + v2) );
    }

    pub fn sub(stack: &mut Vec<f32>) -> () {
	take_two!(stack, v1, v2, stack.push(v1 - v2) );
    }

    pub fn mul(stack: &mut Vec<f32>) -> () {
	take_two!(stack, v1, v2, stack.push(v1 * v2) );
    }

    pub fn div(stack: &mut Vec<f32>) -> () {
	take_two!(stack, v1, v2, stack.push(v1 / v2));
    }

    // idiv
    pub fn idiv(stack: &mut Vec<f32>) -> () {
	take_two!(stack, v1, v2, stack.push((v1 as i32 / v2 as i32) as f32) );
    }

    // mod
    pub fn mod_fn(stack: &mut Vec<f32>) -> () {
	take_two!(stack, v1, v2, stack.push((v1 as i32 % v2 as i32) as f32) );
    }


    // abs
    pub fn abs(stack: &mut Vec<f32>) -> () {
	take_one!(stack, v, stack.push(v.abs()) );
    }

    // neg
    pub fn neg(stack: &mut Vec<f32>) -> () {
	take_one!(stack, v, stack.push(-v) );
    }

    // ceiling
    pub fn ceiling(stack: &mut Vec<f32>) -> () {
	take_one!(stack, v, stack.push(v.ceil()) );
    }

    // floor
    pub fn floor(stack: &mut Vec<f32>) -> () {
	take_one!(stack, v, stack.push(v.floor()) );
    }

    // round
    pub fn round(stack: &mut Vec<f32>) -> () {
	take_one!(stack, v, stack.push(v.round()) );
    }

    // truncate
    pub fn truncate(stack: &mut Vec<f32>) -> () {
	take_one!(stack, v, stack.push(v.trunc()) );
    }

    // sqrt
    pub fn sqrt(stack: &mut Vec<f32>) -> () {
	take_one!(stack, v, stack.push(v.sqrt()) );
    }

    // exp
    pub fn exp(stack: &mut Vec<f32>) -> () {
	take_two!(stack, v1, v2, stack.push(v1.powf(v2)) );
    }

    // ln
    pub fn ln(stack: &mut Vec<f32>) -> () {
	take_one!(stack, v, stack.push(v.ln()) );
    }

    // log
    pub fn log(stack: &mut Vec<f32>) -> () {
	take_one!(stack, v, stack.push(v.log10()) );
    }

    // sin
    pub fn sin(stack: &mut Vec<f32>) -> () {
	take_one!(stack, v, stack.push(v.to_radians().sin()) );
    }

    // cos
    pub fn cos(stack: &mut Vec<f32>) -> () {
	take_one!(stack, v, stack.push(v.to_radians().cos()) );
    }

    // atan
    pub fn atan(stack: &mut Vec<f32>) -> () {
	take_two!(stack, v1, v2, {
            let r = v1.atan2(v2).to_degrees();
            if r >= 0. {
                stack.push(r);
            } else {
                stack.push(r + 360.);
            }
	});
    }

    // rand
    pub fn rand(stack: &mut Vec<f32>) -> () {
        stack.push(rand::random::<u32>() as f32);
    }

    // srand
    // rrand
    // if
    // ifelse
    // exec
    // for
    // repeat
    // loop
    // forall
    // exit

    pub fn eq(stack: &mut Vec<f32>) -> () {
	take_one!(stack, v, println!(" {}", v) );
    }

    // eeq
    pub fn stack_fn(stack: &mut Vec<f32>) -> () {
        for i in stack.iter().rev() {
            println!(" {}", i);
        }
    }

    // pstack

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_dup() {
            let mut stack = vec![1., 2.];
            dup(&mut stack);
            assert_eq!(stack, [1., 2., 2.]);
        }

        #[test]
        fn test_exch() {
            let mut stack = vec![1., 2.];
            exch(&mut stack);
            assert_eq!(stack, [2., 1.]);
        }

        #[test]
        fn test_pop() {
	    let mut stack = vec![1., 2., 3.];
	    pop(&mut stack);
	    assert_eq!(stack, [1., 2.]);
	}

        #[test]
        fn test_copy() {
	    {
		let mut stack = vec![1., 2., 3., 2.];
		copy(&mut stack);
		assert_eq!(stack, [1., 2., 3., 2., 3.]);
	    }
	    {
		let mut stack = vec![1., 2., 3., 0.];
		copy(&mut stack);
		assert_eq!(stack, [1., 2., 3.]);
	    }
	}

        #[test]
        fn test_roll() {
            let mut stack = vec![4., 3., 2., 1., 3., 1.];
            roll(&mut stack);
            assert_eq!(stack, [4., 1., 3., 2.]);
            stack.push(3.);
            stack.push(-1.);
            roll(&mut stack);
            assert_eq!(stack, [4., 3., 2., 1.]);
            stack.push(4.);
            stack.push(2.);
            roll(&mut stack);
            assert_eq!(stack, [2., 1., 4., 3.]);
        }

        #[test]
        fn test_index() {
            {
                let mut stack = vec![1., 2., 3., 4., 0.,];
                index(&mut stack);
                assert_eq!(stack, [1., 2., 3., 4., 4.]);
            }
            {
                let mut stack = vec![1., 2., 3., 4., 3.,];
                index(&mut stack);
                assert_eq!(stack, [1., 2., 3., 4., 1.]);
            }
	}

        #[test]
        fn test_clear() {
            let mut stack = vec![1., 2., 3., 4.];
            clear(&mut stack);
            assert_eq!(stack, []);
	}

        #[test]
        fn test_count() {
            let mut stack = vec![1., 2.];
            count(&mut stack);
            assert_eq!(stack, [1., 2., 2.]);
            stack.clear();
            count(&mut stack);
            assert_eq!(stack, [0.]);
        }

        #[test]
        fn test_add() {
            let mut stack = vec![1., 2.];
            add(&mut stack);
            assert_eq!(stack, [3.]);
        }

        #[test]
        fn test_sub() {
            let mut stack = vec![1., 2.];
            sub(&mut stack);
            assert_eq!(stack, [-1.]);
        }

        #[test]
        fn test_mul() {
            let mut stack = vec![3., 4.];
            mul(&mut stack);
            assert_eq!(stack, [12.]);
        }

        #[test]
        fn test_div() {
            {
                let mut stack = vec![3., 2.];
                div(&mut stack);
                assert_eq!(stack, [1.5]);
            }
            {
                let mut stack = vec![4., 2.];
                div(&mut stack);
                assert_eq!(stack, [2.]);
            }
        }

        #[test]
        fn test_idiv() {
            {
                let mut stack = vec![3., 2.];
                idiv(&mut stack);
                assert_eq!(stack, [1.]);
            }
            {
                let mut stack = vec![4., 2.];
                idiv(&mut stack);
                assert_eq!(stack, [2.]);
            }
            {
                let mut stack = vec![-5., 2.];
                idiv(&mut stack);
                assert_eq!(stack, [-2.]);
            }
        }

        #[test]
        fn test_mod() {
            {
                let mut stack = vec![5., 3.];
                mod_fn(&mut stack);
                assert_eq!(stack, [2.]);
            }
            {
                let mut stack = vec![5., 2.];
                mod_fn(&mut stack);
                assert_eq!(stack, [1.]);
            }
            {
                let mut stack = vec![-5., 3.];
                mod_fn(&mut stack);
                assert_eq!(stack, [-2.]);
            }
	}

        #[test]
        fn test_abs() {
            {
                let mut stack = vec![-2.];
                abs(&mut stack);
                assert_eq!(stack, [2.]);
            }
            {
                let mut stack = vec![3.];
                abs(&mut stack);
                assert_eq!(stack, [3.]);
            }
	}

        #[test]
        fn test_neg() {
            {
                let mut stack = vec![-2.];
                neg(&mut stack);
                assert_eq!(stack, [2.]);
            }
            {
                let mut stack = vec![3.];
                neg(&mut stack);
                assert_eq!(stack, [-3.]);
            }
	}

        #[test]
        fn test_ceiling() {
            {
                let mut stack = vec![3.2];
                ceiling(&mut stack);
                assert_eq!(stack, [4.]);
            }
            {
                let mut stack = vec![-4.8];
                ceiling(&mut stack);
                assert_eq!(stack, [-4.]);
            }
            {
                let mut stack = vec![99.];
                ceiling(&mut stack);
                assert_eq!(stack, [99.]);
            }
	}

        #[test]
        fn test_floor() {
            {
                let mut stack = vec![3.2];
                floor(&mut stack);
                assert_eq!(stack, [3.]);
            }
            {
                let mut stack = vec![-4.8];
                floor(&mut stack);
                assert_eq!(stack, [-5.]);
            }
            {
                let mut stack = vec![99.];
                floor(&mut stack);
                assert_eq!(stack, [99.]);
            }
	}

        #[test]
        fn test_round() {
            {
                let mut stack = vec![3.2];
                round(&mut stack);
                assert_eq!(stack, [3.]);
            }
//            {
//                let mut stack = vec![6.5];
//                round(&mut stack);
//                assert_eq!(stack, [7.]);
//            }
            {
                let mut stack = vec![-4.8];
                round(&mut stack);
                assert_eq!(stack, [-5.]);
            }
//            {
//                let mut stack = vec![-6.5];
//                round(&mut stack);
//                assert_eq!(stack, [-6.]);
//            }
            {
                let mut stack = vec![99.];
                round(&mut stack);
                assert_eq!(stack, [99.]);
            }
	}

        #[test]
        fn test_truncate() {
            {
                let mut stack = vec![3.2];
                truncate(&mut stack);
                assert_eq!(stack, [3.]);
            }
            {
                let mut stack = vec![-4.8];
                truncate(&mut stack);
                assert_eq!(stack, [-4.]);
            }
            {
                let mut stack = vec![99.];
                truncate(&mut stack);
                assert_eq!(stack, [99.]);
            }
	}

        #[test]
        fn test_sqrt() {
            let mut stack = vec![9.];
            sqrt(&mut stack);
            assert_eq!(stack, [3.]);
        }

        #[test]
        fn test_exp() {
            {
                let mut stack = vec![9., 0.5];
                exp(&mut stack);
                assert_eq!(stack, [3.]);
            }
            {
                let mut stack = vec![-9., -1.];
                exp(&mut stack);
                assert_eq!(stack, [-0.11111111]);
            }
	}


        #[test]
        fn test_ln() {
            {
                let mut stack = vec![10.];
                ln(&mut stack);
                assert_eq!(stack, [2.3025851]);
            }
            {
                let mut stack = vec![100.];
                ln(&mut stack);
                assert_eq!(stack, [4.6051702]);
            }
	}

        #[test]
        fn test_log() {
            {
                let mut stack = vec![10.];
                log(&mut stack);
                assert_eq!(stack, [1.]);
            }
            {
                let mut stack = vec![100.];
                log(&mut stack);
                assert_eq!(stack, [2.]);
            }
	}

	// test_sin

        #[test]
        fn test_cos() {
            {
                let mut stack = vec![0.];
                cos(&mut stack);
                assert_eq!(stack, [1.]);
            }
// @todo need something like the approx crate
//          {
//              let mut stack = vec![90.];
//              cos(&mut stack);
//              assert_eq!(stack, [0.]);
//          }
        }

        #[test]
        fn test_atan() {
            {
                let mut stack = vec![0., 1.];
                atan(&mut stack);
                assert_eq!(stack, [0.]);
            }
            {
                let mut stack = vec![1., 0.];
                atan(&mut stack);
                assert_eq!(stack, [90.]);
            }
            {
                let mut stack = vec![-100., 0.];
                atan(&mut stack);
                assert_eq!(stack, [270.]);
            }
            {
                let mut stack = vec![4., 4.];
                atan(&mut stack);
                assert_eq!(stack, [45.]);
            }
        }

	// test_rand
//        #[test]
//        fn test_for() {
//            {
//                let mut stack = vec![0., 1., 1., 4., "{add}", "for"];
//                for(&mut stack);
//                assert_eq!(stack, [10.]);
//            }
	//	}

	// test_eq
	// test_stack
    }
}

fn main() {
    let mut rl = Editor::<()>::new();
    let mut stack = Vec::new();
    let mut function_table: HashMap<String, fn(&mut Vec<f32>)> = HashMap::new();

    use crate::op::*;
    // 3.6.1
    add_op!(function_table, dup);
    add_op!(function_table, exch);
    add_op!(function_table, pop);
    add_op!(function_table, copy);
    add_op!(function_table, roll);
    add_op!(function_table, index);
    add_op!(function_table, clear);
    add_op!(function_table, count);

    add_op!(function_table, add);
    add_op!(function_table, sub);
    add_op!(function_table, mul);
    add_op!(function_table, div);
    add_op!(function_table, idiv);

//    add_op!(function_table, mod);
    function_table.insert("mod".to_string(), op::mod_fn);

    add_op!(function_table, abs);
    add_op!(function_table, neg);
    add_op!(function_table, ceiling);
    add_op!(function_table, floor);
    add_op!(function_table, round);
    add_op!(function_table, truncate);

    // 3.6.2
    add_op!(function_table, sqrt);
    add_op!(function_table, exp);
    add_op!(function_table, ln);
    add_op!(function_table, log);
    add_op!(function_table, sin);
    add_op!(function_table, cos);
    add_op!(function_table, atan);
    add_op!(function_table, rand);

    // 3.6.5
    // if
    // ifelse
    // exec
    // for
    // repeat
    // loop
    // forall
    // exit

    // 3.8.5
    function_table.insert("=".to_string(), op::eq);
    function_table.insert("stack".to_string(), op::stack_fn);

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let words = line.as_str().split(" ");
                for w in words {
                    match w.parse::<f32>() {
                        Ok(val) => {
                            stack.push(val);
                        }
                        _ => {
                            match function_table.get(w) {
                                Some(fcn) => {
                                    fcn(&mut stack)
                                }
                                _ => {
                                    println!("word: {}", w);
                                }
                            }
                        }
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}

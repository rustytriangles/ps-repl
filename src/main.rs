//use rand::prelude::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

mod op {

    // @todo value should probably move out of op
    #[derive(PartialEq, Debug, Copy, Clone)]
    pub enum Tag { Number, Symbol, String }

    pub union U {
	pub f: f32,
	pub s: &'static str
    }

    pub struct Value { pub tag: Tag, pub val: U }

    impl Clone for Value {
	fn clone(&self) -> Self {
	    if self.tag == Tag::Number {
		unsafe {
		    Value { tag: self.tag, val: U { f: self.val.f }}
		}
	    } else {
		unsafe {
		    Value { tag: self.tag, val: U { s: self.val.s }}
		}
	    }
	}
    }

    impl PartialEq for Value {
	fn eq(&self, other: &Self) -> bool {
	    if self.tag == Tag::Number && other.tag == Tag::Number {
		unsafe {
		    self.val.f == other.val.f
		}
	    } else if self.tag == Tag::String && other.tag == Tag::String {
		unsafe {
		    self.val.s == other.val.s
		}
	    } else {
		false
	    }
	}
    }

    impl std::fmt::Debug for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	    unsafe {
		f.debug_struct("Value")
		    .field("tag", &self.tag)
		    .field("val", &self.val.f)
		    .finish()
	    }
	}
    }

    #[macro_export]
    macro_rules! number {
	($ex:expr) => {
	    Value { tag: Tag::Number, val: U { f: $ex } }
	}
    }

    // op table macros
    #[macro_export]
    macro_rules! add_op {
	($table:tt, $func_name:ident) => {
	    $table.insert(stringify!($func_name).to_string(), $func_name);
	};
    }

    macro_rules! take_one_number {
	($s:ident, $v:ident, $ex:expr) => {
	    unsafe {
		match $s.pop() {
		    Some(Value { tag: Tag::Number, val: U { f: $v } }) => {
			$ex;
		    }
		    // @todo add different message for symbol & string
		    _ => {
			println!("Error: stack underflow");
		    }
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

    macro_rules! take_two_numbers {
	($s:ident, $v1:ident, $v2:ident, $ex:expr) => {
	    unsafe {
		match $s.pop() {
		    Some(Value { tag: Tag::Number, val: U { f: $v2 } }) => {
			match $s.pop() {
			    Some(Value { tag: Tag::Number, val: U { f: $v1 } }) => {
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
    }

    pub fn dup(stack: &mut Vec<Value>) -> () {
        match stack.pop() {
	    Some(v) => {
		stack.push(v.clone());
		stack.push(v.clone());
	    }
	    _ => {
                println!("Error: stack underflow");
	    }
	}
    }

    pub fn exch(stack: &mut Vec<Value>) -> () {
	take_two!(stack, v1, v2, {
            stack.push(v2.clone());
            stack.push(v1.clone());
        });
    }

    pub fn pop(stack: &mut Vec<Value>) -> () {
	take_one_number!(stack, _v, {} );
    }

    pub fn copy(stack: &mut Vec<Value>) -> () {
	take_one_number!(stack, v, {
            let n = v as usize;
            if stack.len() >= n {
                let start = stack.len() - n;
                let end = stack.len();
                for i in start..end {
                    stack.push(stack[i].clone());
                }
            }
	});
    }

    pub fn roll(stack: &mut Vec<Value>) -> () {
	take_two_numbers!(stack, v1, v2, {
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
                    stack.push(temp[k%n].clone());
                }
            } else {
                println!("Error: stack underflow");
            }
	});
    }

    pub fn index(stack: &mut Vec<Value>) -> () {
	take_one_number!(stack, v, {
	    let n = v as usize;
            if stack.len() >= n {
                let i = stack.len() - n - 1;
                stack.push(stack[i].clone());
            }
	});
    }

    pub fn clear(stack: &mut Vec<Value>) -> () {
        stack.clear()
    }

    pub fn count(stack: &mut Vec<Value>) -> () {
        stack.push(number!(stack.len() as f32))
    }

    pub fn add(stack: &mut Vec<Value>) -> () {
	take_two_numbers!(stack, v1, v2, stack.push(number!(v1 + v2)))
    }

    pub fn sub(stack: &mut Vec<Value>) -> () {
	take_two_numbers!(stack, v1, v2, stack.push(number!(v1 - v2)))
    }

    pub fn mul(stack: &mut Vec<Value>) -> () {
	take_two_numbers!(stack, v1, v2, stack.push(number!(v1 * v2)))
    }

    pub fn div(stack: &mut Vec<Value>) -> () {
	take_two_numbers!(stack, v1, v2, stack.push(number!(v1 / v2)))
    }

    // idiv
    pub fn idiv(stack: &mut Vec<Value>) -> () {
	take_two_numbers!(stack, v1, v2, stack.push(number!((v1 as i32 / v2 as i32) as f32)))
    }

    // mod
    pub fn mod_fn(stack: &mut Vec<Value>) -> () {
	take_two_numbers!(stack, v1, v2, stack.push(number!((v1 as i32 % v2 as i32) as f32)))
    }


    // abs
    pub fn abs(stack: &mut Vec<Value>) -> () {
	take_one_number!(stack, v, stack.push(number!(v.abs())))
    }

    // neg
    pub fn neg(stack: &mut Vec<Value>) -> () {
	take_one_number!(stack, v, stack.push(number!(-v)))
    }

    // ceiling
    pub fn ceiling(stack: &mut Vec<Value>) -> () {
	take_one_number!(stack, v, stack.push(number!(v.ceil())))
    }

    // floor
    pub fn floor(stack: &mut Vec<Value>) -> () {
	take_one_number!(stack, v, stack.push(number!(v.floor())))
    }

    // round
    pub fn round(stack: &mut Vec<Value>) -> () {
	take_one_number!(stack, v, stack.push(number!(v.round())))
    }

    // truncate
    pub fn truncate(stack: &mut Vec<Value>) -> () {
	take_one_number!(stack, v, stack.push(number!(v.trunc())))
    }

    // sqrt
    pub fn sqrt(stack: &mut Vec<Value>) -> () {
	take_one_number!(stack, v, stack.push(number!(v.sqrt())))
    }

    // exp
    pub fn exp(stack: &mut Vec<Value>) -> () {
	take_two_numbers!(stack, v1, v2, stack.push(number!(v1.powf(v2))))
    }

    // ln
    pub fn ln(stack: &mut Vec<Value>) -> () {
	take_one_number!(stack, v, stack.push(number!(v.ln())))
    }

    // log
    pub fn log(stack: &mut Vec<Value>) -> () {
	take_one_number!(stack, v, stack.push(number!(v.log10())))
    }

    // sin
    pub fn sin(stack: &mut Vec<Value>) -> () {
	take_one_number!(stack, v, stack.push(number!(v.to_radians().sin())))
    }

    // cos
    pub fn cos(stack: &mut Vec<Value>) -> () {
	take_one_number!(stack, v, stack.push(number!(v.to_radians().cos())))
    }

    // atan
    pub fn atan(stack: &mut Vec<Value>) -> () {
	take_two_numbers!(stack, v1, v2, {
            let r = v1.atan2(v2).to_degrees();
            if r >= 0. {
                stack.push(number!(r))
            } else {
                stack.push(number!(r + 360.))
            }
	});
    }

    // rand
    pub fn rand(stack: &mut Vec<Value>) -> () {
        stack.push(number!(rand::random::<u32>() as f32))
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

    pub fn eq(stack: &mut Vec<Value>) -> () {
	unsafe {
            match stack.pop() {
		Some(Value { tag: Tag::Number, val: U { f: v } }) => {
		    println!(" {}", v)
		}
		Some(Value { tag: Tag::String, val: U { s: v } }) => {
		    println!(" {}", v)
		}
		_ => {
		    println!("Error: stack underflow");
		}
	    }
	}
    }

    // eeq
    pub fn stack_fn(stack: &mut Vec<Value>) -> () {
        for i in stack.iter().rev() {
	    unsafe {
		match i {
		    Value { tag: Tag::Number, val: U { f: v } } => {
			println!(" {}", v)
		    }
		    Value { tag: Tag::String, val: U { s: v } } => {
			println!(" {}", v)
		    }
		    _ => {
			println!("Error: stack underflow");
		    }
		}
	    }
	}
    }

    // pstack

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_dup() {
            let mut stack = vec![number!(1.), number!(2.)];
            dup(&mut stack);
            assert_eq!(stack, [number!(1.), number!(2.), number!(2.)]);
        }

        #[test]
        fn test_exch() {
            let mut stack = vec![number!(1.), number!(2.)];
            exch(&mut stack);
            assert_eq!(stack, [number!(2.), number!(1.)]);
        }

        #[test]
        fn test_pop() {
	    let mut stack = vec![number!(1.), number!(2.), number!(3.)];
	    pop(&mut stack);
	    assert_eq!(stack, [number!(1.), number!(2.)]);
	}

        #[test]
        fn test_copy() {
	    {
		let mut stack = vec![number!(1.), number!(2.),
				     number!(3.), number!(2.)];
		copy(&mut stack);
		assert_eq!(stack, [number!(1.), number!(2.), number!(3.),
				   number!(2.), number!(3.)]);
	    }
	    {
		let mut stack = vec![number!(1.), number!(2.), number!(3.),
				     number!(0.)];
		copy(&mut stack);
		assert_eq!(stack, [number!(1.), number!(2.), number!(3.),]);
	    }
	}

        #[test]
        fn test_roll() {
            let mut stack = vec![number!(4.), number!(3.), number!(2.), number!(1.),
				 number!(3.), number!(1.)];
            roll(&mut stack);
            assert_eq!(stack, [number!(4.), number!(1.), number!(3.), number!(2.)]);
            stack.push(number!(3.));
            stack.push(number!(-1.));
            roll(&mut stack);
            assert_eq!(stack, [number!(4.), number!(3.), number!(2.), number!(1.)]);
            stack.push(number!(4.));
            stack.push(number!(2.));
            roll(&mut stack);
            assert_eq!(stack, [number!(2.), number!(1.), number!(4.), number!(3.)]);
        }

        #[test]
        fn test_index() {
            {
                let mut stack = vec![number!(1.), number!(2.), number!(3.), number!(4.),
				     number!(0.)];
                index(&mut stack);
                assert_eq!(stack, [number!(1.), number!(2.), number!(3.), number!(4.),
				   number!(4.)]);
            }
            {
                let mut stack = vec![number!(1.), number!(2.), number!(3.), number!(4.),
				     number!(3.)];
                index(&mut stack);
                assert_eq!(stack, [number!(1.), number!(2.), number!(3.), number!(4.), number!(1.)]);
            }
	}

        #[test]
        fn test_clear() {
            let mut stack = vec![number!(1.), number!(2.), number!(3.), number!(4.)];
            clear(&mut stack);
            assert_eq!(stack, []);
	}

        #[test]
        fn test_count() {
            let mut stack = vec![number!(1.), number!(2.)];
            count(&mut stack);
            assert_eq!(stack, [number!(1.), number!(2.), number!(2.)]);
            stack.clear();
            count(&mut stack);
            assert_eq!(stack, [number!(0.)]);
        }

        #[test]
        fn test_add() {
            let mut stack = vec![number!(1.), number!(2.)];
            add(&mut stack);
            assert_eq!(stack, [number!(3.)]);
        }

        #[test]
        fn test_sub() {
            let mut stack = vec![number!(1.), number!(2.)];
            sub(&mut stack);
            assert_eq!(stack, [number!(-1.)]);
        }

        #[test]
        fn test_mul() {
            let mut stack = vec![number!(3.), number!(4.)];
            mul(&mut stack);
            assert_eq!(stack, [number!(12.)]);
        }

        #[test]
        fn test_div() {
            {
                let mut stack = vec![number!(3.), number!(2.)];
                div(&mut stack);
                assert_eq!(stack, [number!(1.5)]);
            }
            {
                let mut stack = vec![number!(4.), number!(2.)];
                div(&mut stack);
                assert_eq!(stack, [number!(2.)]);
            }
        }

        #[test]
        fn test_idiv() {
            {
                let mut stack = vec![number!(3.), number!(2.)];
                idiv(&mut stack);
                assert_eq!(stack, [number!(1.)]);
            }
            {
                let mut stack = vec![number!(4.), number!(2.)];
                idiv(&mut stack);
                assert_eq!(stack, [number!(2.)]);
            }
            {
                let mut stack = vec![number!(-5.), number!(2.)];
                idiv(&mut stack);
                assert_eq!(stack, [number!(-2.)]);
            }
        }

        #[test]
        fn test_mod() {
            {
                let mut stack = vec![number!(5.), number!(3.)];
                mod_fn(&mut stack);
                assert_eq!(stack, [number!(2.)]);
            }
            {
                let mut stack = vec![number!(5.), number!(2.)];
                mod_fn(&mut stack);
                assert_eq!(stack, [number!(1.)]);
            }
            {
                let mut stack = vec![number!(-5.), number!(3.)];
                mod_fn(&mut stack);
                assert_eq!(stack, [number!(-2.)]);
            }
	}

        #[test]
        fn test_abs() {
            {
                let mut stack = vec![number!(-2.)];
                abs(&mut stack);
                assert_eq!(stack, [number!(2.)]);
            }
            {
                let mut stack = vec![number!(3.)];
                abs(&mut stack);
                assert_eq!(stack, [number!(3.)]);
            }
	}

        #[test]
        fn test_neg() {
            {
                let mut stack = vec![number!(-2.)];
                neg(&mut stack);
                assert_eq!(stack, [number!(2.)]);
            }
            {
                let mut stack = vec![number!(3.)];
                neg(&mut stack);
                assert_eq!(stack, [number!(-3.)]);
            }
	}

        #[test]
        fn test_ceiling() {
            {
                let mut stack = vec![number!(3.2)];
                ceiling(&mut stack);
                assert_eq!(stack, [number!(4.)]);
            }
            {
                let mut stack = vec![number!(-4.8)];
                ceiling(&mut stack);
                assert_eq!(stack, [number!(-4.)]);
            }
            {
                let mut stack = vec![number!(99.)];
                ceiling(&mut stack);
                assert_eq!(stack, [number!(99.)]);
            }
	}

        #[test]
        fn test_floor() {
            {
                let mut stack = vec![number!(3.2)];
                floor(&mut stack);
                assert_eq!(stack, [number!(3.)]);
            }
            {
                let mut stack = vec![number!(-4.8)];
                floor(&mut stack);
                assert_eq!(stack, [number!(-5.)]);
            }
            {
                let mut stack = vec![number!(99.)];
                floor(&mut stack);
                assert_eq!(stack, [number!(99.)]);
            }
	}

        #[test]
        fn test_round() {
            {
                let mut stack = vec![number!(3.2)];
                round(&mut stack);
                assert_eq!(stack, [number!(3.)]);
            }
//            {
//                let mut stack = vec![number!(6.5)];
//                round(&mut stack);
//                assert_eq!(stack, [number!(7.)]);
//            }
            {
                let mut stack = vec![number!(-4.8)];
                round(&mut stack);
                assert_eq!(stack, [number!(-5.)]);
            }
//            {
//                let mut stack = vec![number!(-6.5)];
//                round(&mut stack);
//                assert_eq!(stack, [number!(-6.)]);
//            }
            {
                let mut stack = vec![number!(99.)];
                round(&mut stack);
                assert_eq!(stack, [number!(99.)]);
            }
	}

        #[test]
        fn test_truncate() {
            {
                let mut stack = vec![number!(3.2)];
                truncate(&mut stack);
                assert_eq!(stack, [number!(3.)]);
            }
            {
                let mut stack = vec![number!(-4.8)];
                truncate(&mut stack);
                assert_eq!(stack, [number!(-4.)]);
            }
            {
                let mut stack = vec![number!(99.)];
                truncate(&mut stack);
                assert_eq!(stack, [number!(99.)]);
            }
	}

        #[test]
        fn test_sqrt() {
            let mut stack = vec![number!(9.)];
            sqrt(&mut stack);
            assert_eq!(stack, [number!(3.)]);
        }

        #[test]
        fn test_exp() {
            {
                let mut stack = vec![number!(9.), number!(0.5)];
                exp(&mut stack);
                assert_eq!(stack, [number!(3.)]);
            }
            {
                let mut stack = vec![number!(-9.), number!(-1.)];
                exp(&mut stack);
                assert_eq!(stack, [number!(-0.11111111)]);
            }
	}


        #[test]
        fn test_ln() {
            {
                let mut stack = vec![number!(10.)];
                ln(&mut stack);
                assert_eq!(stack, [number!(2.3025851)]);
            }
            {
                let mut stack = vec![number!(100.)];
                ln(&mut stack);
                assert_eq!(stack, [number!(4.6051702)]);
            }
	}

        #[test]
        fn test_log() {
            {
                let mut stack = vec![number!(10.)];
                log(&mut stack);
                assert_eq!(stack, [number!(1.)]);
            }
            {
                let mut stack = vec![number!(100.)];
                log(&mut stack);
                assert_eq!(stack, [number!(2.)]);
            }
	}

	// test_sin

        #[test]
        fn test_cos() {
            {
                let mut stack = vec![number!(0.)];
                cos(&mut stack);
                assert_eq!(stack, [number!(1.)]);
            }
// @todo need something like the approx crate
//          {
//              let mut stack = vec![number!(90.)];
//              cos(&mut stack);
//              assert_eq!(stack, [number!(0.)]);
//          }
        }

        #[test]
        fn test_atan() {
            {
                let mut stack = vec![number!(0.), number!(1.)];
                atan(&mut stack);
                assert_eq!(stack, [number!(0.)]);
            }
            {
                let mut stack = vec![number!(1.), number!(0.)];
                atan(&mut stack);
                assert_eq!(stack, [number!(90.)]);
            }
            {
                let mut stack = vec![number!(-100.), number!(0.)];
                atan(&mut stack);
                assert_eq!(stack, [number!(270.)]);
            }
            {
                let mut stack = vec![number!(4.), number!(4.)];
                atan(&mut stack);
                assert_eq!(stack, [number!(45.)]);
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
    let mut function_table: HashMap<String, fn(&mut Vec<Value>)> = HashMap::new();

//    let a = NumOrStr::Num(123);
//    let b = NumOrStr::Str("fudge".to_string());
//    print_it(&a);
//    print_it(&b);

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
                            stack.push(number!(val));
                        }
                        _ => {
                            match function_table.get(w) {
                                Some(fcn) => {
                                    fcn(&mut stack)
                                }
                                _ => {
				    // @todo this doesn't handle strings with embedded spaces
				    if w.chars().next().unwrap() == '('
					&& w.chars().last().unwrap() == ')' {
					    let newstr = w.clone();
//					    stack.push(op::Value { tag: op::Tag::String,
//								   val: U { s: newstr }
//					    });
					println!("string: {}", w);
				    } else {
					println!("unknown: {}", w);
				    }
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

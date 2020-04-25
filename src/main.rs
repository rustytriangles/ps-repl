// REPL demo © 2020 RustyTriangles LLC
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

mod op {

    #[derive(Debug,PartialEq)]
    pub enum Value {
        Num(f32),
        Str(String),
        Sym(String)
    }

    impl Clone for Value {
        fn clone(&self) -> Value {
            match self {
                Value::Num(n) => {
                    Value::Num(*n)
                }
                Value::Str(s) => {
                    Value::Str(s.to_string())
                }
                Value::Sym(s) => {
                    Value::Sym(s.to_string())
                }
            }
        }
    }

    #[macro_export]
    macro_rules! number {
        ($ex:expr) => {
            Value::Num($ex)
        }
    }

    #[macro_export]
    macro_rules! string {
        ($ex:expr) => {
            Value::Str($ex.to_string())
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
            match $s.pop() {
                Some(Value::Num($v)) => {
                    $ex;
                }
                Some(Value::Str($v)) => {
                    ignore($v);
                    println!("Error: expected number, found string");
                }
                _ => {
                    println!("Error: stack underflow");
                }
            }
        }
    }

    macro_rules! take_one_string {
        ($s:ident, $v:ident, $ex:expr) => {
            match $s.pop() {
                Some(Value::Str($v)) => {
                    $ex;
                }
                Some(Value::Num($v)) => {
                    ignore($v);
                    println!("Error: expected string, found number");
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

    macro_rules! take_two_numbers {
        ($s:ident, $v1:ident, $v2:ident, $ex:expr) => {
            match $s.pop() {
                Some(Value::Num($v2)) => {
                    match $s.pop() {
                        Some(Value::Num($v1)) => {
                            $ex;
                        }
                        Some(Value::Str($v1)) => {
                            ignore($v1);
                            println!("Error: expected number, found string");
                        }
                        _ => {
                            println!("Error: stack underflow");
                        }
                    }
                }
                Some(Value::Str($v2)) => {
                    ignore($v2);
                    println!("Error: expected number, found string ");
                }
                _ => {
                    println!("Error: stack underflow");
                }
            }
        }
    }

    macro_rules! take_two_strings {
        ($s:ident, $s1:ident, $s2:ident, $ex:expr) => {
            match $s.pop() {
                Some(Value::Str($s2)) => {
                    match $s.pop() {
                        Some(Value::Str($s1)) => {
                            $ex;
                        }
                        Some(Value::Num($s1)) => {
                            ignore($s1);
                            println!("Error: expected number, found number");
                        }
                        _ => {
                            println!("Error: stack underflow");
                        }
                    }
                }
                Some(Value::Num($s2)) => {
                    ignore($s2);
                    println!("Error: expected number, found number ");
                }
                _ => {
                    println!("Error: stack underflow");
                }
            }
        }
    }

    // There are a couple of places where the '_' doesn't work
    // because we're in a macro. This is a lame workaround.
    fn ignore<T>(_: T) {
    }

    //
    // The functions
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
            stack.push(v2);
            stack.push(v1);
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

    pub fn idiv(stack: &mut Vec<Value>) -> () {
        take_two_numbers!(stack, v1, v2, stack.push(number!((v1 as i32 / v2 as i32) as f32)))
    }

    pub fn mod_fn(stack: &mut Vec<Value>) -> () {
        take_two_numbers!(stack, v1, v2, stack.push(number!((v1 as i32 % v2 as i32) as f32)))
    }

    pub fn abs(stack: &mut Vec<Value>) -> () {
        take_one_number!(stack, v, stack.push(number!(v.abs())))
    }

    pub fn neg(stack: &mut Vec<Value>) -> () {
        take_one_number!(stack, v, stack.push(number!(-v)))
    }

    pub fn ceiling(stack: &mut Vec<Value>) -> () {
        take_one_number!(stack, v, stack.push(number!(v.ceil())))
    }

    pub fn floor(stack: &mut Vec<Value>) -> () {
        take_one_number!(stack, v, stack.push(number!(v.floor())))
    }

    pub fn round(stack: &mut Vec<Value>) -> () {
        take_one_number!(stack, v, stack.push(number!(v.round())))
    }

    pub fn truncate(stack: &mut Vec<Value>) -> () {
        take_one_number!(stack, v, stack.push(number!(v.trunc())))
    }

    pub fn sqrt(stack: &mut Vec<Value>) -> () {
        take_one_number!(stack, v, stack.push(number!(v.sqrt())))
    }

    pub fn exp(stack: &mut Vec<Value>) -> () {
        take_two_numbers!(stack, v1, v2, stack.push(number!(v1.powf(v2))))
    }

    pub fn ln(stack: &mut Vec<Value>) -> () {
        take_one_number!(stack, v, stack.push(number!(v.ln())))
    }

    pub fn log(stack: &mut Vec<Value>) -> () {
        take_one_number!(stack, v, stack.push(number!(v.log10())))
    }

    pub fn sin(stack: &mut Vec<Value>) -> () {
        take_one_number!(stack, v, stack.push(number!(v.to_radians().sin())))
    }

    pub fn cos(stack: &mut Vec<Value>) -> () {
        take_one_number!(stack, v, stack.push(number!(v.to_radians().cos())))
    }

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

    pub fn rand(stack: &mut Vec<Value>) -> () {
        stack.push(number!(rand::random::<u32>() as f32))
    }

    pub fn search(stack: &mut Vec<Value>) -> () {
        take_two_strings!(stack, s1, s2, {
            match s1.find(&s2) {
                Some(c) => {
                    let (first, rest) = s1.split_at(c);
                    let (mid, last) = rest.split_at(s1.len() - c -1);
                    stack.push(string!(last));
                    stack.push(string!(mid));
                    stack.push(string!(first));
                    stack.push(number!(1.));

                }
                _ => {
                    stack.push(string!(s1));
                    stack.push(number!(0.));
                }
            }
        });
    }

    pub fn length(stack: &mut Vec<Value>) -> () {
        take_one_string!(stack, s, {
            stack.push(number!(s.len() as f32));
        });
    }

    pub fn eq(stack: &mut Vec<Value>) -> () {
        match stack.pop() {
            Some(Value::Num(v)) => {
                println!(" {}", v)
            }
            Some(Value::Str(s)) => {
                println!(" ({})", s)
            }
            _ => {
                println!("Error: stack underflow");
            }
        }
    }

    pub fn stack_fn(stack: &mut Vec<Value>) -> () {
        for i in stack.iter().rev() {
            match i {
                Value::Num(v) => {
                    println!(" {}", v)
                }
                Value::Str(s) => {
                    println!(" ({})", s)
                }
                _ => {
                    println!("Error: stack underflow");
                }
            }
        }
    }

    //
    // Unittests for the functions
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_dup() {
            {
                let mut stack = vec![number!(1.), number!(2.)];
                dup(&mut stack);
                assert_eq!(stack, [number!(1.), number!(2.), number!(2.)]);
            }
            {
                let mut stack = vec![number!(1.), string!("fred")];
                dup(&mut stack);
                assert_eq!(stack, [number!(1.), string!("fred"), string!("fred")]);
            }
        }

        #[test]
        fn test_exch() {
            {
                let mut stack = vec![number!(1.), number!(2.)];
                exch(&mut stack);
                assert_eq!(stack, [number!(2.), number!(1.)]);
            }
            {
                let mut stack = vec![string!("fred"), string!("ginger")];
                exch(&mut stack);
                assert_eq!(stack, [string!("ginger"), string!("fred")]);
            }
        }

        #[test]
        fn test_pop() {
            {
                let mut stack = vec![number!(1.), number!(2.), number!(3.)];
                pop(&mut stack);
                assert_eq!(stack, [number!(1.), number!(2.)]);
            }
            {
                let mut stack = vec![number!(1.), string!("ducks")];
                pop(&mut stack);
                assert_eq!(stack, [number!(1.)]);
            }
        }

        #[test]
        fn test_copy() {
            {
                let mut stack = vec![number!(1.), number!(2.), number!(3.),
                                     number!(2.)];
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
            {
                let mut stack = vec![number!(1.), string!("monkey"), number!(3.),
                                     number!(2.)];
                copy(&mut stack);
                assert_eq!(stack, [number!(1.), string!("monkey"), number!(3.),
                                   string!("monkey"), number!(3.)]);
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
            // @todo Have an issue with rounding dir for 1/2
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

        #[test]
        fn test_search() {
            {
                let mut stack = vec![string!("abbc"), string!("bb")];
                search(&mut stack);
                // retval should really be true
                assert_eq!(stack, vec![string!("c"), string!("bb"), string!("a"), number!(1.)]);
            }
        }

        #[test]
        fn test_length() {
            {
                let mut stack = vec![string!("abbc")];
                length(&mut stack);
                assert_eq!(stack, vec![number!(4.)]);
            }
        }

        //        #[test]
        //        fn test_for() {
        //            {
        //                let mut stack = vec![0., 1., 1., 4., "{add}", "for"];
        //                for(&mut stack);
        //                assert_eq!(stack, [10.]);
        //            }
        //      }

        // test_eq
        // test_stack
    }
}

fn main() {
    // Editor for reading stdin
    let mut rl = Editor::<()>::new();
    // The stack
    let mut stack = Vec::new();
    // Table mapping function names to function implementations
    let mut function_table: HashMap<String, fn(&mut Vec<Value>)> = HashMap::new();

    use crate::op::*;

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
    add_op!(function_table, abs);
    add_op!(function_table, neg);
    add_op!(function_table, ceiling);
    add_op!(function_table, floor);
    add_op!(function_table, round);
    add_op!(function_table, truncate);
    add_op!(function_table, sqrt);
    add_op!(function_table, exp);
    add_op!(function_table, ln);
    add_op!(function_table, log);
    add_op!(function_table, sin);
    add_op!(function_table, cos);
    add_op!(function_table, atan);
    add_op!(function_table, rand);
    add_op!(function_table, search);
    add_op!(function_table, length);

    // Can't use macro for these because their names aren't valid Rust
    function_table.insert("mod".to_string(), op::mod_fn);
    function_table.insert("=".to_string(), op::eq);
    function_table.insert("stack".to_string(), op::stack_fn);

    // The actual REPL
    loop {
        // Read the next line
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                // Break it into words
                let words = line.as_str().split(" ");
                // For each word ...
                for w in words {
                    // If we can read it as a f32, then it's a number.
                    // Push it onto the stack.
                    match w.parse::<f32>() {
                        Ok(val) => {
                            stack.push(Value::Num(val));
                        }
                        _ => {
                            // Otherwise, look in the function table
                            match function_table.get(w) {
                                // If it's a valid function, execute it.
                                Some(fcn) => {
                                    fcn(&mut stack)
                                }
                                _ => {
                                    // Otherwise, if 1st & last chars are parens, it's a string.
                                    //
                                    // @todo This doesn't handle strings with embedded spaces
                                    // because the split will already have chopped them up.
                                    if w.chars().next().unwrap() == '('
                                        && w.chars().last().unwrap() == ')' {
                                            stack.push(Value::Str(w
                                                                  .trim_start_matches('(')
                                                                  .trim_end_matches(')')
                                                                  .to_string()));
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

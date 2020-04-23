//use rand::prelude::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

mod op {
    pub fn dup(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v) => {
                stack.push(v);
                stack.push(v);
            }
            _ => {
                println!("Error: stack underflow");
            }
        }
    }

    pub fn exch(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v1) => {
                match stack.pop() {
                    Some(v2) => {
                        stack.push(v2);
                        stack.push(v1);
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

    pub fn pop(stack: &mut Vec<f32>) -> () {
        if stack.len() >= 1 {
            stack.pop();
        } else {
            println!("Error: stack underflow");
        }
    }

    pub fn copy(stack: &mut Vec<f32>) -> () {
        if stack.len() >= 1 {
            let n = stack.pop().unwrap() as usize;
            if stack.len() >= n {
                let start = stack.len() - n;
                let end = stack.len();
                for i in start..end {
                    stack.push(stack[i]);
                }
            } else {
                println!("Error: stack underflow");
            }

        } else {
            println!("Error: stack underflow");
        }
    }

    pub fn roll(stack: &mut Vec<f32>) -> () {
        if stack.len() >= 2 {
            let i = stack.pop().unwrap() as i32;
            let n = stack.pop().unwrap() as usize;
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

        } else {
            println!("Error: stack underflow");
        }
    }

    pub fn index(stack: &mut Vec<f32>) -> () {
        if stack.len() >= 1 {
            let n = stack.pop().unwrap() as usize;
            if stack.len() >= n {
                let i = stack.len() - n;
                stack.push(stack[i]);
            } else {
                println!("Error: stack underflow");
            }

        } else {
            println!("Error: stack underflow");
        }
    }

    pub fn clear(stack: &mut Vec<f32>) -> () {
        stack.clear();
    }

    pub fn count(stack: &mut Vec<f32>) -> () {
        stack.push(stack.len() as f32);
    }

    pub fn add(stack: &mut Vec<f32>) -> () {
        if stack.len() >= 2 {
            let v2 = stack.pop().unwrap();
            let v1 = stack.pop().unwrap();
            stack.push(v1 + v2);

        } else {
            println!("Error: stack underflow");
        }
    }

    pub fn sub(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v2) => {
                match stack.pop() {
                    Some(v1) => {
                        stack.push(v1 - v2);
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

    pub fn mul(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v1) => {
                match stack.pop() {
                    Some(v2) => {
                        stack.push(v1 * v2);
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

    pub fn div(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v2) => {
                match stack.pop() {
                    Some(v1) => {
                        stack.push(v1 / v2);
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

    // idiv
    pub fn idiv(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v2) => {
                match stack.pop() {
                    Some(v1) => {
                        stack.push((v1 as i32 / v2 as i32) as f32);
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

    // mod
    pub fn mod_fn(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v2) => {
                match stack.pop() {
                    Some(v1) => {
                        stack.push((v1 as i32 % v2 as i32) as f32);
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

    // abs
    pub fn abs(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v) => {
                stack.push(v.abs());
            }
            _ => {
                println!("Error: stack underflow");
            }
        }
    }

    // neg
    pub fn neg(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v) => {
                stack.push(-v);
            }
            _ => {
                println!("Error: stack underflow");
            }
        }
    }

    // ceiling
    pub fn ceiling(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v) => {
                stack.push(v.ceil());
            }
            _ => {
                println!("Error: stack underflow");
            }
        }
    }

    // floor
    pub fn floor(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v) => {
                stack.push(v.floor());
            }
            _ => {
                println!("Error: stack underflow");
            }
        }
    }

    // round
    pub fn round(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v) => {
                stack.push(v.round());
            }
            _ => {
                println!("Error: stack underflow");
            }
        }
    }

    // truncate
    pub fn truncate(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v) => {
                stack.push(v.trunc());
            }
            _ => {
                println!("Error: stack underflow");
            }
        }
    }

    // sqrt
    pub fn sqrt(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v) => {
                stack.push(v.sqrt());
            }
            _ => {
                println!("Error: stack underflow");
            }
        }
    }

    // exp
    pub fn exp(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v1) => {
                match stack.pop() {
                    Some(v2) => {
                        stack.push(v2.powf(v1));
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

    // ln
    pub fn ln(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v) => {
                stack.push(v.ln());
            }
            _ => {
                println!("Error: stack underflow");
            }
        }
    }

    // log
    pub fn log(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v) => {
                stack.push(v.log10());
            }
            _ => {
                println!("Error: stack underflow");
            }
        }
    }

    // sin
    pub fn sin(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v) => {
                stack.push(v.to_radians().sin());
            }
            _ => {
                println!("Error: stack underflow");
            }
        }
    }

    // cos
    pub fn cos(stack: &mut Vec<f32>) -> () {
        match stack.pop() {
            Some(v) => {
                stack.push(v.to_radians().cos());
            }
            _ => {
                println!("Error: stack underflow");
            }
        }
    }

    // atan
    pub fn atan(stack: &mut Vec<f32>) -> () {
        if stack.len() >= 2 {
            let v2 = stack.pop().unwrap();
            let v1 = stack.pop().unwrap();
            let r = v1.atan2(v2).to_degrees();
            if r >= 0. {
                stack.push(r);
            } else {
                stack.push(r + 360.);
            }

        } else {
            println!("Error: stack underflow");
        }
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
        match stack.pop() {
            Some(v) => {
                println!(" {}", v);
            }
            _ => {
                println!("Error: stack underflow");
            }
        }
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

//        #[test]
//        fn test_for() {
//            {
//                let mut stack = vec![0., 1., 1., 4., "{add}", "for"];
//                for(&mut stack);
//                assert_eq!(stack, [10.]);
//            }
//	}
    }
}

fn main() {
    let mut rl = Editor::<()>::new();
    let mut stack = Vec::new();
    let mut function_table: HashMap<String, fn(&mut Vec<f32>)> = HashMap::new();

    // 3.6.1
    function_table.insert("dup".to_string(), op::dup);
    function_table.insert("exch".to_string(), op::exch);
    function_table.insert("pop".to_string(), op::pop);
    function_table.insert("copy".to_string(), op::copy);
    function_table.insert("roll".to_string(), op::roll);
    function_table.insert("index".to_string(), op::index);
//    function_table.insert("mark".to_string(), op::mark);
    function_table.insert("clear".to_string(), op::clear);
    function_table.insert("count".to_string(), op::count);

    function_table.insert("add".to_string(), op::add);
    function_table.insert("sub".to_string(), op::sub);
    function_table.insert("mul".to_string(), op::mul);
    function_table.insert("div".to_string(), op::div);
    function_table.insert("idiv".to_string(), op::idiv);
    function_table.insert("mod".to_string(), op::mod_fn);

    function_table.insert("abs".to_string(), op::abs);
    function_table.insert("neg".to_string(), op::neg);
    function_table.insert("ceiling".to_string(), op::ceiling);
    function_table.insert("floor".to_string(), op::floor);
    function_table.insert("round".to_string(), op::round);
    function_table.insert("truncate".to_string(), op::truncate);

    // 3.6.2
    function_table.insert("sqrt".to_string(), op::sqrt);
    function_table.insert("exp".to_string(), op::exp);
    function_table.insert("ln".to_string(), op::ln);
    function_table.insert("log".to_string(), op::log);
    function_table.insert("sin".to_string(), op::sin);
    function_table.insert("cos".to_string(), op::cos);
    function_table.insert("atan".to_string(), op::atan);
    function_table.insert("rand".to_string(), op::rand);
//    function_table.insert("srand".to_string(), op::srand);
//    function_table.insert("rrand".to_string(), op::rrand);

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
//    function_table.insert("==".to_string(), op::eeq);
    function_table.insert("stack".to_string(), op::stack_fn);
//    function_table.insert("pstack".to_string(), op::pstack);

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

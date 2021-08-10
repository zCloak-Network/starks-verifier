use starksVM::{ ProgramInputs, assembly };
use super::{ Example, utils::parse_args };

pub fn get_example(args: &[String]) -> Example  {

    // read starting value of the sequence and proof options from the arguments
    let (value, options) = parse_args(args);


    // determine the expected result
    let expected_result: u128 = add_result(value);
    
    // construct the program which executes an unbounded loop to compute a Collatz sequence
    // which starts with the provided value; the output of the program is the number of steps
    // needed to reach the end of the sequence

    let program = assembly::compile("
    begin 
    push.0 push.1 push.2 push.4 
    push.1 read dup push.1 
    while.true
        roll.4 ne 
        if.true 
            swap push.1 add dup push.5 ne 
            if.true 
                swap dup push.1
            else
                pad.3 
            end
        else 
            push.1 pad
        end
    end
end").unwrap();

    println!("Generated a program to judge a value if 0 1 2 4 ; expected result: {}", 
        expected_result);
    
        let inputs = ProgramInputs::new(&[], &[value as u128], &[]);

    // put the starting value as the only secret input for tape A

    // a single element from the top of the stack will be the output
    let num_outputs = 1;

    return Example {
        program,
        inputs,
        options,
        expected_result: vec![expected_result],
        num_outputs
    };
}

/// Computes number of steps in a Collatz sequence which starts with the provided `value`.



fn add_result(value: usize) -> u128{
        let v = vec![0,1,2,4];
        for i in v.iter(){
            if *i == value {
                return 1;
            }else{   }
        };
        return 0;
}


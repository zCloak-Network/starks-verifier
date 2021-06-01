

// RE-EXPORTS
// ================================================================================================


pub mod opcodes;
pub use opcodes::{ UserOps as OpCode, OpHint };

// PUBLIC FUNCTIONS
// ================================================================================================



// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {

    use crate::{ programs::assembly, stark::TraceState, utils::as_bytes };
    use super::{ ProgramInputs };

    #[test]
    fn execute_span() {
        let program = assembly::compile("begin add push.5 mul push.7 end").unwrap();
        let inputs = ProgramInputs::from_public(&[1, 2]);

        let (trace, ctx_depth, loop_depth) = super::execute(&program, &inputs);
        let trace_length = trace[0].len();

        assert_eq!(64, trace_length);
        assert_eq!(17, trace.len());
        let mut state = build_trace_state(trace.len(), ctx_depth, loop_depth) ;
        state.update_from_trace(&trace, trace_length - 1);

        assert_eq!(46, state.op_counter());
        assert_eq!(program.hash(), as_bytes(state.program_hash()));
        assert_eq!([1, 1, 1], state.cf_op_bits());
        assert_eq!([1, 1, 1, 1, 1], state.ld_op_bits());
        assert_eq!([1, 1], state.hd_op_bits());
        assert_eq!([0], state.ctx_stack());
        assert_eq!([7, 15, 0, 0, 0, 0, 0, 0], state.user_stack());
    }

    #[test]
    fn execute_block() {
        let program = assembly::compile("begin add block push.5 mul push.7 end end").unwrap();
        let inputs = ProgramInputs::from_public(&[1, 2]);

        let (trace, ctx_depth, loop_depth) = super::execute(&program, &inputs);
        let trace_length = trace[0].len();

        assert_eq!(64, trace_length);
        assert_eq!(18, trace.len());

        let mut state = build_trace_state(trace.len(), ctx_depth, loop_depth) ;
        state.update_from_trace(&trace, trace_length - 1);
        
        assert_eq!(60, state.op_counter());
        assert_eq!(program.hash(), as_bytes(state.program_hash()));
        assert_eq!([1, 1, 1], state.cf_op_bits());
        assert_eq!([1, 1, 1, 1, 1], state.ld_op_bits());
        assert_eq!([1, 1], state.hd_op_bits());
        assert_eq!([0], state.ctx_stack());
        assert_eq!([0], state.loop_stack());
        assert_eq!([7, 15, 0, 0, 0, 0, 0, 0], state.user_stack());
    }

    #[test]
    fn execute_if_else() {
        let program = assembly::compile(
            "begin read if.true add push.3 else push.7 add push.8 end mul end").unwrap();
        
        // execute true branch
        let inputs = ProgramInputs::new(&[5, 3], &[1], &[]);
        let (trace, ctx_depth, loop_depth) = super::execute(&program, &inputs);
        let trace_length = trace[0].len();

        assert_eq!(128, trace_length);
        assert_eq!(19, trace.len());

        let mut state = build_trace_state(trace.len(), ctx_depth, loop_depth) ;
        state.update_from_trace(&trace, trace_length - 1);

        assert_eq!(76, state.op_counter());
        assert_eq!(program.hash(), as_bytes(state.program_hash()));
        assert_eq!([1, 1, 1], state.cf_op_bits());
        assert_eq!([1, 1, 1, 1, 1], state.ld_op_bits());
        assert_eq!([1, 1], state.hd_op_bits());
        assert_eq!([0], state.ctx_stack());
        assert_eq!([0], state.loop_stack());
        assert_eq!([24, 0, 0, 0, 0, 0, 0, 0], state.user_stack());

        // execute false branch
        let inputs = ProgramInputs::new(&[5, 3], &[0], &[]);
        let (trace, ctx_depth, loop_depth) = super::execute(&program, &inputs);
        let trace_length = trace[0].len();

        assert_eq!(128, trace_length);
        assert_eq!(19, trace.len());

        let mut state = build_trace_state(trace.len(), ctx_depth, loop_depth) ;
        state.update_from_trace(&trace, trace_length - 1);

        assert_eq!(92, state.op_counter());
        assert_eq!(program.hash(), as_bytes(state.program_hash()));
        assert_eq!([1, 1, 1], state.cf_op_bits());
        assert_eq!([1, 1, 1, 1, 1], state.ld_op_bits());
        assert_eq!([1, 1], state.hd_op_bits());
        assert_eq!([0], state.ctx_stack());
        assert_eq!([0], state.loop_stack());
        assert_eq!([96, 3, 0, 0, 0, 0, 0, 0], state.user_stack());
    }

    #[test]
    fn execute_loop() {
        let program = assembly::compile(
            "begin mul read while.true dup mul read end end").unwrap();

        // don't enter the loop
        let inputs = ProgramInputs::new(&[5, 3], &[0], &[]);
        let (trace, ctx_depth, loop_depth) = super::execute(&program, &inputs);
        let trace_length = trace[0].len();

        assert_eq!(64, trace_length);
        assert_eq!(18, trace.len());

        let mut state = build_trace_state(trace.len(), ctx_depth, loop_depth) ;
        state.update_from_trace(&trace, trace_length - 1);

        assert_eq!(60, state.op_counter());
        assert_eq!(program.hash(), as_bytes(state.program_hash()));
        assert_eq!([1, 1, 1], state.cf_op_bits());
        assert_eq!([1, 1, 1, 1, 1], state.ld_op_bits());
        assert_eq!([1, 1], state.hd_op_bits());
        assert_eq!([0], state.ctx_stack());
        assert_eq!([0], state.loop_stack());
        assert_eq!([15, 0, 0, 0, 0, 0, 0, 0], state.user_stack());

        // execute one iteration
        let inputs = ProgramInputs::new(&[5, 3], &[1, 0], &[]);
        let (trace, ctx_depth, loop_depth) = super::execute(&program, &inputs);
        let trace_length = trace[0].len();

        assert_eq!(128, trace_length);
        assert_eq!(19, trace.len());

        let mut state = build_trace_state(trace.len(), ctx_depth, loop_depth) ;
        state.update_from_trace(&trace, trace_length - 1);

        assert_eq!(75, state.op_counter());
        assert_eq!(program.hash(), as_bytes(state.program_hash()));
        assert_eq!([1, 1, 1], state.cf_op_bits());
        assert_eq!([1, 1, 1, 1, 1], state.ld_op_bits());
        assert_eq!([1, 1], state.hd_op_bits());
        assert_eq!([0], state.ctx_stack());
        assert_eq!([0], state.loop_stack());
        assert_eq!([225, 0, 0, 0, 0, 0, 0, 0], state.user_stack());

        // execute five iteration
        let inputs = ProgramInputs::new(&[5, 3], &[1, 1, 1, 1, 1, 0], &[]);
        let (trace, ctx_depth, loop_depth) = super::execute(&program, &inputs);
        let trace_length = trace[0].len();

        assert_eq!(256, trace_length);
        assert_eq!(19, trace.len());

        let mut state = build_trace_state(trace.len(), ctx_depth, loop_depth) ;
        state.update_from_trace(&trace, trace_length - 1);

        assert_eq!(135, state.op_counter());
        assert_eq!(program.hash(), as_bytes(state.program_hash()));
        assert_eq!([1, 1, 1], state.cf_op_bits());
        assert_eq!([1, 1, 1, 1, 1], state.ld_op_bits());
        assert_eq!([1, 1], state.hd_op_bits());
        assert_eq!([0], state.ctx_stack());
        assert_eq!([0], state.loop_stack());
        assert_eq!([43143988327398919500410556793212890625, 0, 0, 0, 0, 0, 0, 0], state.user_stack());
    }

    fn build_trace_state(num_registers: usize, ctx_depth: usize, loop_depth: usize) -> TraceState {
        let decoder_width = TraceState::compute_decoder_width(ctx_depth, loop_depth);
        let stack_depth = num_registers - decoder_width;
        return TraceState::new(ctx_depth, loop_depth, stack_depth);
    }
}
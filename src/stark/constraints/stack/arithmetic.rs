use super::{
    are_equal, binary_not, enforce_left_shift, enforce_stack_copy, field, is_between, is_binary,
    EvaluationResult,
};
use sp_std::{vec, vec::Vec};

// ARITHMETIC OPERATION
// ================================================================================================

/// Enforces constraints for ADD operation. The constraints are based on the first 2 elements of
/// the stack; the rest of the stack is shifted left by 1 element.
pub fn enforce_add(result: &mut [u128], old_stack: &[u128], new_stack: &[u128], op_flag: u128) {
    let x = old_stack[0];
    let y = old_stack[1];
    let op_result = field::add(x, y);
    result.agg_constraint(0, op_flag, are_equal(new_stack[0], op_result));

    // ensure that the rest of the stack is shifted by 1 item to the left
    enforce_left_shift(result, old_stack, new_stack, 2, 1, op_flag);
}

/// Enforces constraints for MUL operation. The constraints are based on the first 2 elements of
/// the stack; the rest of the stack is shifted left by 1 element.
pub fn enforce_mul(result: &mut [u128], old_stack: &[u128], new_stack: &[u128], op_flag: u128) {
    let x = old_stack[0];
    let y = old_stack[1];
    let op_result = field::mul(x, y);
    result.agg_constraint(0, op_flag, are_equal(new_stack[0], op_result));

    // ensure that the rest of the stack is shifted by 1 item to the left
    enforce_left_shift(result, old_stack, new_stack, 2, 1, op_flag);
}

/// Enforces constraints for INV operation. The constraints are based on the first element of
/// the stack; the rest of the stack is unaffected.
pub fn enforce_inv(result: &mut [u128], old_stack: &[u128], new_stack: &[u128], op_flag: u128) {
    // Constraints for INV operation is defined as: x * inv(x) = 1; this also means
    // that if x = 0, the constraint will not be satisfied
    let x = old_stack[0];
    let inv_x = new_stack[0];
    result.agg_constraint(0, op_flag, are_equal(field::ONE, field::mul(inv_x, x)));

    // ensure nothing changed beyond the first item of the stack
    enforce_stack_copy(result, old_stack, new_stack, 1, op_flag);
}

/// Enforces constraints for NEG operation. The constraints are based on the first element of
/// the stack; the rest of the stack is unaffected.
pub fn enforce_neg(result: &mut [u128], old_stack: &[u128], new_stack: &[u128], op_flag: u128) {
    // Constraint for NEG operation is defined as: x + neg(x) = 0
    let x = old_stack[0];
    let neg_x = new_stack[0];
    result.agg_constraint(0, op_flag, field::add(neg_x, x));

    // ensure nothing changed beyond the first item of the stack
    enforce_stack_copy(result, old_stack, new_stack, 1, op_flag);
}

// BOOLEAN OPERATION
// ================================================================================================

/// Enforces constraints for NOT operation. The constraints are based on the first element of
/// the stack, but also evaluates an auxiliary constraint which guarantees that the first
/// element of the stack is binary.
pub fn enforce_not(
    result: &mut [u128],
    aux: &mut [u128],
    old_stack: &[u128],
    new_stack: &[u128],
    op_flag: u128,
) {
    // NOT operation is defined simply as: 1 - x; this means 0 becomes 1, and 1 becomes 0
    let x = old_stack[0];
    let op_result = binary_not(x);
    result.agg_constraint(0, op_flag, are_equal(new_stack[0], op_result));

    // ensure nothing changed beyond the first item of the stack
    enforce_stack_copy(result, old_stack, new_stack, 1, op_flag);

    // we also need to make sure that the operand is binary (i.e. 0 or 1)
    aux.agg_constraint(0, op_flag, is_binary(x));
}

/// Enforces constraints for AND operation. The constraints are based on the first two elements
/// of the stack, but also evaluates auxiliary constraints which guarantee that both elements
/// are binary.
pub fn enforce_and(
    result: &mut [u128],
    aux: &mut [u128],
    old_stack: &[u128],
    new_stack: &[u128],
    op_flag: u128,
) {
    // AND operation is the same as: x * y
    let x = old_stack[0];
    let y = old_stack[1];
    let op_result = field::mul(x, y);
    result.agg_constraint(0, op_flag, are_equal(new_stack[0], op_result));

    // ensure that the rest of the stack is shifted by 1 item to the left
    enforce_left_shift(result, old_stack, new_stack, 2, 1, op_flag);

    // ensure that both operands are binary values
    aux.agg_constraint(0, op_flag, is_binary(x));
    aux.agg_constraint(1, op_flag, is_binary(y));
}

/// Enforces constraints for OR operation. The constraints are based on the first two elements
/// of the stack, but also evaluates auxiliary constraints which guarantee that both elements
/// are binary.
pub fn enforce_or(
    result: &mut [u128],
    aux: &mut [u128],
    old_stack: &[u128],
    new_stack: &[u128],
    op_flag: u128,
) {
    // OR operation is the same as: 1 - (1 - x) * (1 - y)
    let x = old_stack[0];
    let y = old_stack[1];
    let op_result = binary_not(field::mul(binary_not(x), binary_not(y)));
    result.agg_constraint(0, op_flag, are_equal(new_stack[0], op_result));

    // ensure that the rest of the stack is shifted by 1 item to the left
    enforce_left_shift(result, old_stack, new_stack, 2, 1, op_flag);

    // ensure that both operands are binary values
    aux.agg_constraint(0, op_flag, is_binary(x));
    aux.agg_constraint(1, op_flag, is_binary(y));
}

/// Enforces constraints for Kvalid operation. The constraints are based on 9 elements of
/// the stack; the rest of the stack is unaffected.
pub fn enforce_kvalid(result: &mut [u128], old_stack: &[u128], new_stack: &[u128], op_flag: u128) {
    // let kvalid_a = new_stack[0];
    // let kvalid_b = new_stack[1];

    // ensure that the rest of the stack is shifted by 7 item to the left
    enforce_left_shift(result, old_stack, new_stack, 9, 7, op_flag);
}

/// Enforces constraints for Khash operation. The constraints are based on 9 elements of
/// the stack; the rest of the stack is unaffected.
pub fn enforce_khash(result: &mut [u128], old_stack: &[u128], new_stack: &[u128], op_flag: u128) {
    // let mut hash_in_khash: Vec<u128> = Vec::new();
    let n = old_stack[0];
    let is_between = is_between(2, 15, n);


    if is_between == field::ZERO && (2 * n as usize) <= old_stack.len() {
        // for i in 0..n {
        //     let k = (2 * (n - 1) - 2 * i) as usize;
        //     hash_in_khash.push(old_stack[k + 1]);
        //     hash_in_khash.push(old_stack[k + 2]);
        // }

        enforce_left_shift(
            result,
            old_stack,
            new_stack,
            (2 * n + 1) as usize,
            (2 * n - 1) as usize,
            op_flag,
        );
    }
}

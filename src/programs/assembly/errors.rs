extern crate alloc;
use alloc::string::String;
// TYPES AND INTERFACES
// ================================================================================================
pub struct AssemblyError {
    message: String,
    step: usize,
    op: String,
}

// ASSEMBLY ERROR IMPLEMENTATION
// ================================================================================================
impl AssemblyError {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    pub fn empty_program() -> AssemblyError {
        return AssemblyError {
            message: String::from("a program must contain at least one instruction"),
            step: 0,
            op: String::from("begin"),
        };
    }

    pub fn empty_block(op: &[&str], step: usize) -> AssemblyError {
        return AssemblyError {
            message: String::from("a program block must contain at least one instruction"),
            step,
            op: op.join("."),
        };
    }

    pub fn invalid_program_start(op: &str) -> AssemblyError {
        return AssemblyError {
            message: String::from("a program must start with a 'being' instruction"),
            step: 0,
            op: String::from(op),
        };
    }

    pub fn invalid_program_end(op: &str) -> AssemblyError {
        return AssemblyError {
            message: String::from("a program must end with an 'end' instruction"),
            step: 0,
            op: String::from(op),
        };
    }

    pub fn dangling_instructions(step: usize) -> AssemblyError {
        return AssemblyError {
            message: format!("dangling instructions after program end"),
            step,
            op: String::from("end"),
        };
    }

    pub fn invalid_op(op: &[&str], step: usize) -> AssemblyError {
        return AssemblyError {
            message: format!("instruction {} is invalid", op.join(".")),
            step,
            op: op.join("."),
        };
    }

    pub fn missing_param(op: &[&str], step: usize) -> AssemblyError {
        return AssemblyError {
            message: format!("malformed instruction {}: parameter is missing", op[0]),
            step,
            op: op.join("."),
        };
    }

    pub fn extra_param(op: &[&str], step: usize) -> AssemblyError {
        return AssemblyError {
            message: format!(
                "malformed instruction {}: too many parameters provided",
                op[0]
            ),
            step,
            op: op.join("."),
        };
    }

    pub fn invalid_param(op: &[&str], step: usize) -> AssemblyError {
        return AssemblyError {
            message: format!(
                "malformed instruction {}: parameter '{}' is invalid",
                op[0], op[1]
            ),
            step,
            op: op.join("."),
        };
    }

    pub fn invalid_param_reason(op: &[&str], step: usize, reason: String) -> AssemblyError {
        return AssemblyError {
            message: format!("malformed instruction {}: {}", op[0], reason),
            step,
            op: op.join("."),
        };
    }

    pub fn invalid_block_head(op: &[&str], step: usize) -> AssemblyError {
        return AssemblyError {
            message: format!("invalid block head '{}'", op.join(".")),
            step,
            op: op.join("."),
        };
    }

    pub fn invalid_num_iterations(op: &[&str], step: usize) -> AssemblyError {
        return AssemblyError {
            message: format!(
                "invalid repeat statement '{}': 2 or more iterations must be specified",
                op.join(".")
            ),
            step,
            op: op.join("."),
        };
    }

    pub fn dangling_else(step: usize) -> AssemblyError {
        return AssemblyError {
            message: format!("else without matching if"),
            step,
            op: String::from("else"),
        };
    }

    pub fn unmatched_block(step: usize) -> AssemblyError {
        return AssemblyError {
            message: format!("block without matching end"),
            step,
            op: String::from("block"),
        };
    }

    pub fn unmatched_if(step: usize) -> AssemblyError {
        return AssemblyError {
            message: format!("if without matching else/end"),
            step,
            op: String::from("if.true"),
        };
    }

    pub fn unmatched_while(step: usize) -> AssemblyError {
        return AssemblyError {
            message: format!("while without matching end"),
            step,
            op: String::from("while.true"),
        };
    }

    pub fn unmatched_repeat(step: usize, op: &[&str]) -> AssemblyError {
        return AssemblyError {
            message: format!("repeat without matching end"),
            step,
            op: op.join("."),
        };
    }

    pub fn unmatched_else(step: usize) -> AssemblyError {
        return AssemblyError {
            message: format!("else without matching end"),
            step,
            op: String::from("else"),
        };
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------
    pub fn message(&self) -> &String {
        return &self.message;
    }

    pub fn operation(&self) -> &String {
        return &self.op;
    }

    pub fn step(&self) -> usize {
        return self.step;
    }
}

// COMMON TRAIT IMPLEMENTATIONS
// ================================================================================================

impl sp_std::fmt::Debug for AssemblyError {
    fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
        write!(f, "assembly error at {}: {}", self.step, self.message)
    }
}

impl sp_std::fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
        write!(f, "assembly error at {}: {}", self.step, self.message)
    }
}

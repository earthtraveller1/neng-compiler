// This is definitely necessary and will be useful for much later
// For simplicity, all statements will assume that the data pointer is at 0

use super::assembler::Instruction;

pub trait CodeGen {
    fn go_to_addr(&mut self, addr: u64);
    fn go_to_addr_from_addr(&mut self, begin: u64, end: u64);
    fn go_to_zero_from_addr(&mut self, addr: u64);
    fn set_to_zero(&mut self, addr: u64);

    fn assignment_statement(&mut self, destination_address: u64, value: u8);
    fn copy_statement(&mut self, destination: u64, source: u64);

    fn add(&mut self, destination: u64, value: u8);
    fn add_var(&mut self, destination: u64, source: u64);
    fn subtract(&mut self, destination: u64, value: u8);
    fn subtract_var(&mut self, destination: u64, source: u64);

    fn begin_while_loop(&mut self, variable: u64);
    fn end_while_loop(&mut self, variable: u64);

    fn begin_if(&mut self, variable: u64);
    fn end_if(&mut self, variable: u64);

    fn print(&mut self, variable: u64);
    fn read_into(&mut self, variable: u64);
}

impl CodeGen for Vec<Instruction> {
    fn go_to_addr(&mut self, addr: u64) {
        for _ in 0..addr {
            self.push(Instruction::MoveRight);
        }
    }

    fn go_to_addr_from_addr(&mut self, begin: u64, end: u64) {
        let begin: i64 = begin.try_into().unwrap();
        let end: i64 = end.try_into().unwrap();

        let distance = end - begin;

        if distance < 0 {
            for _ in 0..(distance.abs()) {
                self.push(Instruction::MoveLeft);
            }
        } else if distance > 0 {
            for _ in 0..(distance.abs()) {
                self.push(Instruction::MoveRight);
            }
        }

        // Obviously if the distance is 0 this is a no op
    }

    fn go_to_zero_from_addr(&mut self, addr: u64) {
        for _ in 0..addr {
            self.push(Instruction::MoveLeft);
        }
    }

    fn set_to_zero(&mut self, addr: u64) {
        self.go_to_addr(addr);
        self.push(Instruction::JumpForwardIf0);
        self.push(Instruction::Decrement);
        self.push(Instruction::JumpBackwardIfNot0);
        self.go_to_zero_from_addr(addr);
    }

    fn assignment_statement(&mut self, destination_address: u64, value: u8) {
        self.go_to_addr(destination_address);

        // Reset the value at the address to 0, first and foremost.
        self.push(Instruction::JumpForwardIf0);
        self.push(Instruction::Decrement);
        self.push(Instruction::JumpBackwardIfNot0);

        // And finally set the value
        for _ in 0..=value {
            self.push(Instruction::Increment);
        }

        self.go_to_zero_from_addr(destination_address);
    }

    fn copy_statement(&mut self, destination: u64, source: u64) {
        // Because of this, there will have to be gaps in between the variables
        // so that this can be possible
        let temp = source + 1;

        // First we have to ensure that both temp and destination are 0.
        self.set_to_zero(temp);
        self.set_to_zero(destination);

        self.go_to_addr(source);
        
        // Basically, we now move and duplicate the source value two times
        self.push(Instruction::JumpForwardIf0);
        self.push(Instruction::Decrement);
        self.go_to_addr_from_addr(source, temp);
        self.push(Instruction::Increment);
        self.go_to_addr_from_addr(temp, destination);
        self.push(Instruction::Increment);
        self.go_to_addr_from_addr(destination, source);
        self.push(Instruction::JumpBackwardIfNot0);

        // And now we move the temporary value (which holds the source value) 
        // back to the source place.
        self.go_to_addr_from_addr(source, temp);
        self.push(Instruction::JumpForwardIf0);
        self.push(Instruction::Decrement);
        self.go_to_addr_from_addr(temp, source);
        self.push(Instruction::Increment);
        self.go_to_addr_from_addr(source, temp);
        self.push(Instruction::JumpBackwardIfNot0);

        // Of course we will have to move back to 0 in the end no matter what
        self.go_to_zero_from_addr(temp);
    }

    fn add(&mut self, destination: u64, value: u8) {
        self.go_to_addr(destination);

        for _ in 0..value {
            self.push(Instruction::Increment);
        }

        self.go_to_zero_from_addr(destination);
    }

    fn add_var(&mut self, destination: u64, source: u64) {
        let temp1 = source + 1;
        let temp2 = destination + 1;

        self.set_to_zero(temp1);
        self.set_to_zero(temp2);

        // Move and duplicate the source value
        self.go_to_addr(source);
        self.push(Instruction::JumpForwardIf0);
        self.push(Instruction::Decrement);
        self.go_to_addr_from_addr(source, temp1);
        self.push(Instruction::Increment);
        self.go_to_addr_from_addr(temp1, temp2);
        self.push(Instruction::Increment);
        self.go_to_addr_from_addr(temp2, source);
        self.push(Instruction::JumpBackwardIfNot0);

        // Add temp2 to the destination (since temp1 is closer to source 
        // and we will use that to restore the source value)
        self.go_to_addr_from_addr(source, temp2);
        self.push(Instruction::JumpForwardIf0);
        self.push(Instruction::Decrement);
        self.go_to_addr_from_addr(temp2, destination);
        self.push(Instruction::Increment);
        self.go_to_addr_from_addr(destination, temp2);
        self.push(Instruction::JumpBackwardIfNot0);

        // And finally use temp1 to restore the source value in its place
        self.go_to_addr_from_addr(temp2, temp1);
        self.push(Instruction::JumpForwardIf0);
        self.push(Instruction::Decrement);
        self.go_to_addr_from_addr(temp1, source);
        self.push(Instruction::Increment);
        self.go_to_addr_from_addr(source, temp1);
        self.push(Instruction::JumpBackwardIfNot0);

        self.go_to_zero_from_addr(temp1);
    }

    fn subtract(&mut self, destination: u64, value: u8) {
        self.go_to_addr(destination);

        for _ in 0..value {
            self.push(Instruction::Decrement);
        }

        self.go_to_zero_from_addr(destination);
    }

    fn subtract_var(&mut self, destination: u64, source: u64) {
        let temp1 = source + 1;
        let temp2 = destination + 1;

        self.set_to_zero(temp1);
        self.set_to_zero(temp2);

        // Move and duplicate the source value
        self.go_to_addr(source);
        self.push(Instruction::JumpForwardIf0);
        self.push(Instruction::Decrement);
        self.go_to_addr_from_addr(source, temp1);
        self.push(Instruction::Increment);
        self.go_to_addr_from_addr(temp1, temp2);
        self.push(Instruction::Increment);
        self.go_to_addr_from_addr(temp2, source);
        self.push(Instruction::JumpBackwardIfNot0);

        // Subtract temp2 from the destination (since temp1 is closer to source 
        // and we will use that to restore the source value)
        self.go_to_addr_from_addr(source, temp2);
        self.push(Instruction::JumpForwardIf0);
        self.push(Instruction::Decrement);
        self.go_to_addr_from_addr(temp2, destination);
        self.push(Instruction::Decrement);
        self.go_to_addr_from_addr(destination, temp2);
        self.push(Instruction::JumpBackwardIfNot0);

        // And finally use temp1 to restore the source value in its place
        self.go_to_addr_from_addr(temp2, temp1);
        self.push(Instruction::JumpForwardIf0);
        self.push(Instruction::Decrement);
        self.go_to_addr_from_addr(temp1, source);
        self.push(Instruction::Increment);
        self.go_to_addr_from_addr(source, temp1);
        self.push(Instruction::JumpBackwardIfNot0);

        self.go_to_zero_from_addr(temp1);
    }

    fn begin_while_loop(&mut self, variable: u64) {
        self.go_to_addr(variable);
        self.push(Instruction::JumpForwardIf0);
        self.go_to_zero_from_addr(variable);
    }

    fn end_while_loop(&mut self, variable: u64) {
        self.go_to_addr(variable);
        // Here we don't need to go back to 0 since when we loop around to the 
        // start of the loop it will end up at 0 automatically
        self.push(Instruction::JumpBackwardIfNot0);
        self.go_to_zero_from_addr(variable);
    }

    fn begin_if(&mut self, variable: u64) {
        self.go_to_addr(variable);
        self.push(Instruction::JumpForwardIf0);
        self.go_to_zero_from_addr(variable);
    }

    fn end_if(&mut self, variable: u64) {
        self.set_to_zero(variable + 1);
        self.go_to_addr(variable + 1);

        self.push(Instruction::JumpBackwardIfNot0);
        self.go_to_zero_from_addr(variable + 1);
    }

    fn print(&mut self, variable: u64) {
        self.go_to_addr(variable);
        self.push(Instruction::Print);
        self.go_to_zero_from_addr(variable);
    }

    fn read_into(&mut self, variable: u64) {
        self.go_to_addr(variable);
        self.push(Instruction::Read);
        self.go_to_zero_from_addr(variable);
    }
}

pub mod template;

// Use this file to add helper functions and additional modules.

pub mod helper {

    pub fn grid_size(input: &str) -> (usize, usize) {
        let y = input.split_terminator('\n').count();
        let x = input.split_terminator('\n').next().unwrap().chars().count();
        (x, y)
    }
}

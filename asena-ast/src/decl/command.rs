use crate::{
    BodyWalker, Command, DeclWalker, Expr, ExprWalker, PatWalker, PropertyWalker, StmtWalker,
};

pub type Result<T = ()> = std::result::Result<T, CommandError>;

#[derive(Debug)]
pub enum CommandError {
    InvalidArgument { expected_node_type: &'static str },
}

impl Command {
    /// Checks if the command is the given argument.
    pub fn is_command(&self, _arg: &str) -> bool {
        todo!()
    }

    pub fn at<T: TryFrom<Expr>>(&self, nth: usize) -> Result<T> {
        self.arguments()
            .get(nth)
            .cloned()
            .and_then(|x| x.try_into().ok())
            .ok_or(CommandError::InvalidArgument {
                expected_node_type: std::any::type_name::<T>(),
            })
    }
}

pub trait CommandWalker: BodyWalker + PropertyWalker + ExprWalker + PatWalker + StmtWalker {
    fn on_command(&mut self, _value: &Command) -> Result {
        Ok(())
    }
}

impl<T: CommandWalker> DeclWalker for T {
    fn walk_decl_command(&mut self, value: &Command) {
        self.on_command(value).unwrap();
    }
}

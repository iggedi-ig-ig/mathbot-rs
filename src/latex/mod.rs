pub mod generator;

use serenity::utils::MessageBuilder;

pub struct LaTeXFormula {
    formula: String,
}

impl LaTeXFormula {
    pub fn render(&self) -> MessageBuilder {
        todo!()
    }
}

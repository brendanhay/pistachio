use std::convert::Infallible;

use crate::{
    render::{
        Context,
        Render,
        Writer,
    },
    Template,
};

pub struct Source {
    pub source: String,
}

impl Render for Source {
    #[inline]
    fn render(&self, context: Context, writer: &mut Writer) -> Result<(), Infallible> {
        let template = Template::new(&self.source).unwrap();
        let context = context.fork(&template.nodes);

        context.render(writer)
    }
}

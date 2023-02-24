use crate::{
    error::Error,
    render::{
        Context,
        Render,
        Writer,
    },
    Template,
};

// #[derive(Debug)]
// pub struct Source {
//     pub source: String,
// }

// impl Render for Source {
//     #[inline]
//     fn render_escaped(&self, context: Context, writer: &mut Writer) -> Result<(), Error> {
//         let template = Template::new(&self.source)?;
//         let context = context.fork(template.nodes());

//         context.render_to_writer(writer)
//     }
// }

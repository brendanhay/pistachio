#[derive(Debug, Copy, Clone)]
pub struct Stack<T> {
    a: Option<T>,
    b: Option<T>,
    c: Option<T>,
    d: Option<T>,
    e: Option<T>,
    f: Option<T>,
    g: Option<T>,
    h: Option<T>,
}

impl<T: Copy> Stack<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            a: None,
            b: None,
            c: None,
            d: None,
            e: None,
            f: None,
            g: None,
            h: None,
        }
    }

    #[inline]
    pub fn view(&self) -> [Option<T>; 8] {
        [
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h,
        ]
    }

    #[inline]
    pub fn push(self, frame: T) -> Self {
        Self {
            a: Some(frame),
            b: self.a,
            c: self.b,
            d: self.c,
            e: self.d,
            f: self.e,
            g: self.f,
            h: self.g,
        }
    }

    #[inline]
    pub fn pop(self) -> Self {
        Self {
            a: self.b,
            b: self.c,
            c: self.d,
            d: self.e,
            e: self.f,
            f: self.g,
            g: self.h,
            h: None,
        }
    }
}

// macro_rules! iter {
//     ( $self:expr, $callback:expr ) => {{
//         let Stack {
//             a,
//             b,
//             c,
//             d,
//             e,
//             f,
//             g,
//             h,
//         } = $self;

//         if $callback(a)?
//             || $callback(b)?
//             || $callback(c)?
//             || $callback(d)?
//             || $callback(e)?
//             || $callback(f)?
//             || $callback(g)?
//             || $callback(h)?
//         {
//             Ok(true)
//         } else {
//             Ok(false)
//         }
//     }};
// }

// macro_rules! iter_context {
//     ( $self:expr, $context:expr, $callback:expr ) => {{
//          let Stack {
//             a,
//             b,
//             c,
//             d,
//             e,
//             f,
//             g,
//             h,
//         } = $self;

//         iter_context!($self, $context, $callback, a b c d e f g h)
//     }};

//     ( $self:expr, $context:expr, $callback:expr, $( $field:expr )* ) => {{
//         let context = $context;

//         $(
//             if $callback($field, context)? {
//                 return Ok(true);
//             }

//             let context = $context.pop();
//         )*

//         Ok(false)
//     }};
// }

// impl RenderKey for Stack<&Data> {
//     fn render_key<W: Writer>(
//         &self,
//         key: &str,
//         escape: Escape,
//         writer: &mut W,
//     ) -> Result<bool, W::Error> {
//         // Try to render this key starting at the top of the stack and
//         // short circuit with success when any frame succeeds.
//         iter!(self, |data| RenderKey::render_key(
//             data, key, escape, writer
//         ))
//     }

//     fn render_section_key<W: Writer>(
//         &self,
//         key: &str,
//         context: Context,
//         writer: &mut W,
//     ) -> Result<bool, W::Error> {
//         // Try to render this key starting at the top of the stack, popping
//         // off each frame to pass to nested sections, short circuiting with
//         // success when any frame succeeds.
//         iter_context!(self, context, |data, context| {
//             RenderKey::render_section_key(data, key, context, writer)
//         })
//     }

//     fn render_inverted_key<W: Writer>(
//         &self,
//         key: &str,
//         context: Context,
//         writer: &mut W,
//     ) -> Result<bool, W::Error> {
//         let result = iter!(self, |data| {
//             RenderKey::render_inverted_key(data, key, context, writer)
//         })?;

//         if !result {
//             context.render(writer);
//             Ok(true)
//         } else {
//             Ok(result)
//         }
//     }
// }

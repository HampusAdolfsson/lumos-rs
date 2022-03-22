use super::RenderBuffer;
use futures::stream::{ BoxStream, StreamExt };

/// A generically-typed stream of [RenderBuffer]s.
pub type BufferStream<'a> = BoxStream<'a, RenderBuffer>;


/// Transforms a stream of [RenderBuffer]s.
///
/// This may be a simple mapping operation, e.g. increasing the saturation of each value in each buffer of the stream.
///
/// It could also be asynchronous and more complex, such as outputting a moving average of buffers at a lower rate than the input stream.
pub trait BufferStreamTransformation<'a> {
    /// Applies the transformation to the input stream, returning a transformed stream.
    ///
    /// For simple transformations a call to [StreamExt::map] should suffice, but asynchronous transformations
    /// might need to implement [futures::stream::Stream].
    fn transform(&self, input: BufferStream<'a>) -> BufferStream<'a>;
}


/// Synchronously applies `f` to each buffer of `input` and outputs the result.
pub fn map<'a, F>(stream: BufferStream<'a>, f: F) -> BufferStream<'a>
        where F: FnMut(RenderBuffer) -> RenderBuffer + Clone + Send + 'a {
    MapTransformation{ f }.transform(stream)
}

/// [BufferStreamTransformation] for [map].
struct MapTransformation<F: FnMut(RenderBuffer) -> RenderBuffer + Clone + Send> {
    f: F,
}
impl<'a, F> BufferStreamTransformation<'a> for MapTransformation<F>
        where F: FnMut(RenderBuffer) -> RenderBuffer + Clone + Send + 'a {
    fn transform(&self, input: BufferStream<'a>) -> BufferStream<'a> {
        input.map(self.f.clone()).boxed()
    }
}
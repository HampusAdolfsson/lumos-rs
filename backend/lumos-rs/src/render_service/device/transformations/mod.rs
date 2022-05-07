pub mod color;
pub mod audio;

use std::marker::PhantomData;

use ::color::{RgbF32, HsvF32};
use futures::stream::{ BoxStream, StreamExt };

/// A generically-typed (i.e. boxed) stream of (color) buffers
pub type BufferStream<'a, T> = BoxStream<'a, Vec<T>>;
type RgbBufferStream<'a> = BufferStream<'a, RgbF32>;
type HsvBufferStream<'a> = BufferStream<'a, HsvF32>;


/// Transforms a stream of color buffers.
///
/// This may be a simple mapping operation, e.g. increasing the saturation of each value in each buffer of the stream.
///
/// It could also be asynchronous and more complex, such as outputting a moving average of buffers at a lower rate than the input stream.
///
/// `I` and `O` represent the input and output color types, respectively.
pub trait BufferStreamTransformation<'a, I, O> {
    /// Applies the transformation to the input stream, returning a transformed stream.
    ///
    /// For simple transformations a call to [StreamExt::map] should suffice, but asynchronous transformations
    /// might need to implement [futures::stream::Stream].
    fn transform(self, input: BufferStream<'a, I>) -> BufferStream<'a, O>;
}


/// Synchronously applies the function `f` to each buffer of `stream` and outputs the result.
fn map<'a, F, I, O>(stream: BufferStream<'a, I>, f: F) -> BufferStream<'a, O> where
        I: 'a,
        O: 'a,
        F: FnMut(Vec<I>) -> Vec<O> + Clone + Send + 'a {
    MapTransformation{
        f,
        i: PhantomData,
        o: PhantomData,
    }.transform(stream)
}

/// [BufferStreamTransformation] for [map].
struct MapTransformation<I, O, F: FnMut(Vec<I>) -> Vec<O> + Clone + Send> {
    f: F,
    i: PhantomData<I>,
    o: PhantomData<O>,
}
impl<'a, I, O, F> BufferStreamTransformation<'a, I, O> for MapTransformation<I, O, F> where
    I: 'a,
    O: 'a,
    F: FnMut(Vec<I>) -> Vec<O> + Clone + Send + 'a {
    fn transform(self, input: BufferStream<'a, I>) -> BufferStream<'a, O> {
        input.map(self.f).boxed()
    }
}
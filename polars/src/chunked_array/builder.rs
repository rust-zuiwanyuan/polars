use crate::prelude::*;
use arrow::array::{Array, PrimitiveBuilder, StringBuilder};
use arrow::datatypes::{ArrowPrimitiveType, Field};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct PrimitiveChunkedBuilder<T>
where
    T: ArrowPrimitiveType,
{
    pub builder: PrimitiveBuilder<T>,
    capacity: usize,
    field: Field,
}

impl<T> PrimitiveChunkedBuilder<T>
where
    T: ArrowPrimitiveType,
{
    pub fn new(name: &str, capacity: usize) -> Self {
        PrimitiveChunkedBuilder {
            builder: PrimitiveBuilder::<T>::new(capacity),
            capacity,
            field: Field::new(name, T::get_data_type(), true),
        }
    }

    pub fn new_from_iter(mut self, it: impl Iterator<Item = Option<T::Native>>) -> ChunkedArray<T> {
        it.for_each(|opt| self.append_option(opt).expect("could not append"));
        self.finish()
    }

    pub fn finish(mut self) -> ChunkedArray<T> {
        let arr = Arc::new(self.builder.finish());
        let len = arr.len();
        ChunkedArray {
            field: Arc::new(self.field),
            chunks: vec![arr],
            chunk_id: vec![len],
            phantom: PhantomData,
        }
    }
}

impl<T: ArrowPrimitiveType> Deref for PrimitiveChunkedBuilder<T> {
    type Target = PrimitiveBuilder<T>;

    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}

impl<T: ArrowPrimitiveType> DerefMut for PrimitiveChunkedBuilder<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.builder
    }
}

pub struct Utf8ChunkedBuilder {
    pub builder: StringBuilder,
    capacity: usize,
    field: Field,
}

impl Utf8ChunkedBuilder {
    pub fn new(name: &str, capacity: usize) -> Self {
        Utf8ChunkedBuilder {
            builder: StringBuilder::new(capacity),
            capacity,
            field: Field::new(name, ArrowDataType::Utf8, true),
        }
    }

    pub fn finish(mut self) -> Utf8Chunked {
        let arr = Arc::new(self.builder.finish());
        let len = arr.len();
        ChunkedArray {
            field: Arc::new(self.field),
            chunks: vec![arr],
            chunk_id: vec![len],
            phantom: PhantomData,
        }
    }
}

impl Deref for Utf8ChunkedBuilder {
    type Target = StringBuilder;

    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}

impl DerefMut for Utf8ChunkedBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.builder
    }
}

pub fn build_primitive_ca_with_opt<T>(s: &[Option<T::Native>], name: &str) -> ChunkedArray<T>
where
    T: ArrowPrimitiveType,
    T::Native: Copy,
{
    let mut builder = PrimitiveChunkedBuilder::new(name, s.len());
    for opt in s {
        builder.append_option(*opt).expect("could not append");
    }
    let ca = builder.finish();
    ca
}

use std::marker::PhantomData;

use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ReadableStream, ReadableStreamDefaultReader, ReadableStreamReadResult, js_sys};

use crate::{Error, PromiseExt};

/// A wrapper around ReadableStream
pub struct Reader<T: JsCast> {
	inner: ReadableStreamDefaultReader,

	// Keep the most recent promise to make `read` cancelable
	read: Option<JsFuture>,

	_phantom: PhantomData<T>,
}

impl<T: JsCast> Reader<T> {
	/// Grab a lock on the given readable stream until dropped.
	pub fn new(stream: &ReadableStream) -> Result<Self, Error> {
		let inner = stream.get_reader().unchecked_into();
		Ok(Self {
			inner,
			read: None,
			_phantom: PhantomData,
		})
	}

	/// Read the next element from the stream, returning None if the stream is done.
	pub async fn read(&mut self) -> Result<Option<T>, Error> {
		if self.read.is_none() {
			self.read = Some(JsFuture::from(self.inner.read()));
		}

		let result: ReadableStreamReadResult = self.read.as_mut().unwrap().await?.into();
		self.read.take(); // Clear the promise on success

		if Reflect::get(&result, &"done".into())?.is_truthy() {
			return Ok(None);
		}

		let res = Reflect::get(&result, &"value".into())?.unchecked_into();

		Ok(Some(res))
	}

	/// Abort the stream early with the given reason.
	pub fn abort(&mut self, reason: &str) {
		let str = JsValue::from_str(reason);
		self.inner.cancel_with_reason(&str).ignore();
	}

	pub async fn closed(&self) -> Result<(), Error> {
		JsFuture::from(self.inner.closed()).await?;
		Ok(())
	}
}

impl<T: JsCast> Drop for Reader<T> {
	/// Release the lock
	fn drop(&mut self) {
		self.inner.release_lock();
	}
}

#[cfg(feature = "tokio")]
mod tokio_impl {
	use super::*;
	use crate::reader::js_sys::Uint8Array;
	use ErrorKind::Other;
	use Poll::{Pending, Ready};
	use js_sys::Promise;
	use std::future::Future;
	use std::io::Result;
	use std::io::{Error, ErrorKind};
	use std::pin::Pin;
	use std::task::{Context, Poll};
	use tokio::io::{AsyncRead, ReadBuf};

	impl AsyncRead for Reader<Uint8Array> {
		fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<Result<()>> {
			//if there is no pending read, we need to create one
			if self.read.is_none() {
				self.read = Some(JsFuture::from(self.inner.read()));
			}

			let Some(promise) = self.read.as_mut() else {
				return Ready(Err(Error::new(Other, "no pending read found")));
			};

			match Pin::new(promise).poll(cx) {
				Pending => Pending,
				Ready(Ok(js_val)) => {
					self.read.take();

					let result = js_val.unchecked_into::<ReadableStreamReadResult>();
					if result.get_done().unwrap_or(false) {
						return Ready(Ok(())); // EOF
					}

					let Ok(array) = result.get_value().dyn_into::<Uint8Array>() else {
						return Ready(Err(Error::new(Other, "expected js Uint8Array")));
					};
					let array_len = array.length() as usize;
					let len = std::cmp::min(buf.remaining(), array_len);

					let dst = unsafe { &mut buf.unfilled_mut()[0..len] };
					array.slice(0, len as u32).copy_to_uninit(dst);
					unsafe {
						buf.assume_init(len);
					}
					buf.advance(len);

					if len < array_len {
						let leftover = array.slice(len as u32, array_len as u32);
						result.set_done(false);
						result.set_value(&leftover);
						let promise = Promise::resolve(&**result);
						self.read = Some(JsFuture::from(promise));
					}

					Ready(Ok(()))
				}
				Ready(Err(err)) => {
					self.read.take();
					let msg = format!("{:?}", err);
					Ready(Err(Error::new(Other, msg)))
				}
			}
		}
	}
}

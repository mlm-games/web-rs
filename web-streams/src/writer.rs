use std::marker::PhantomData;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{WritableStream, WritableStreamDefaultWriter};

use crate::{Error, PromiseExt};

// Wrapper around WritableStream
pub struct Writer {
	inner: WritableStreamDefaultWriter,
}

impl Writer {
	pub fn new(stream: &WritableStream) -> Result<Self, Error> {
		let inner = stream.get_writer()?.unchecked_into();
		Ok(Self { inner })
	}

	pub async fn write(&mut self, v: &JsValue) -> Result<(), Error> {
		JsFuture::from(self.inner.write_with_chunk(v)).await?;
		Ok(())
	}

	pub fn close(&mut self) {
		self.inner.close().ignore();
	}

	pub fn abort(&mut self, reason: &str) {
		let str = JsValue::from_str(reason);
		self.inner.abort_with_reason(&str).ignore();
	}

	pub async fn closed(&self) -> Result<(), Error> {
		JsFuture::from(self.inner.closed()).await?;
		Ok(())
	}
}

impl Drop for Writer {
	fn drop(&mut self) {
		self.inner.release_lock();
	}
}

impl<T: JsCast> From<Writer> for TypedWriter<T> {
	fn from(value: Writer) -> Self {
		let inner = value.inner.clone();
		// Forget the original to avoid double release_lock.
		std::mem::forget(value);

		TypedWriter {
			inner,
			write_promise: None,
			_phantom: PhantomData,
		}
	}
}

impl<T: JsCast> Drop for TypedWriter<T> {
	fn drop(&mut self) {
		self.inner.release_lock();
	}
}

impl<T: JsCast> TryFrom<TypedWriter<T>> for Writer {
	type Error = TypedWriter<T>;

	fn try_from(value: TypedWriter<T>) -> Result<Self, Self::Error> {
		if value.write_promise.is_some() {
			Err(value)
		} else {
			let inner = value.inner.clone();
			std::mem::forget(value);
			Ok(Writer { inner })
		}
	}
}

pub struct TypedWriter<T: JsCast> {
	inner: WritableStreamDefaultWriter,
	// Keep the most recent promise to make `write` cancelable
	write_promise: Option<JsFuture>,

	_phantom: PhantomData<T>,
}

impl<T: JsCast> TypedWriter<T> {
	pub fn new(stream: &WritableStream) -> Result<Self, Error> {
		let inner = stream.get_writer()?.unchecked_into();
		Ok(Self {
			inner,
			write_promise: None,
			_phantom: PhantomData,
		})
	}

	pub async fn write(&mut self, v: &T) -> Result<(), Error> {
		if let Some(promise) = &mut self.write_promise.take() {
			promise.await?;
		}
		let js_value = JsValue::from(v);
		self.write_promise = Some(JsFuture::from(self.inner.write_with_chunk(&js_value)));
		if let Some(promise) = &mut self.write_promise {
			promise.await?;
			self.write_promise = None;
		}
		Ok(())
	}

	pub fn close(&mut self) {
		self.inner.close().ignore();
	}

	pub fn abort(&mut self, reason: &str) {
		let str = JsValue::from_str(reason);
		self.inner.abort_with_reason(&str).ignore();
	}

	pub async fn closed(&self) -> Result<(), Error> {
		JsFuture::from(self.inner.closed()).await?;
		Ok(())
	}
}

#[cfg(feature = "tokio")]
mod tokio_impl {
	use super::*;
	use ErrorKind::{BrokenPipe, Other};
	use std::future::Future;
	use std::io::Result;
	use std::io::{Error, ErrorKind};
	use std::pin::Pin;
	use std::task::Poll::{Pending, Ready};
	use std::task::{Context, Poll};
	use tokio::io::AsyncWrite;
	use web_sys::js_sys::Uint8Array;

	impl AsyncWrite for TypedWriter<Uint8Array> {
		fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
			let Ok(Some(desired_size)) = self.inner.desired_size() else {
				return Ready(Err(Error::new(BrokenPipe, "stream is closed")));
			};

			let this = self.get_mut();
			let write_promise = &mut this.write_promise;

			// Stream has backpressure and there's an in-flight write: wait for it
			if desired_size < 1f64 {
				if let Some(promise) = write_promise {
					match Pin::new(promise).poll(cx) {
						Pending => return Pending,
						Ready(Ok(_)) => *write_promise = None,
						Ready(Err(err)) => {
							*write_promise = None;
							let msg = err.as_string().unwrap_or_else(|| format!("{:?}", err));
							return Ready(Err(Error::new(Other, msg)));
						}
					}
				}
			}

			let array = Uint8Array::from(buf);
			*write_promise = Some(JsFuture::from(this.inner.write_with_chunk(&array)));
			Ready(Ok(buf.len()))
		}

		fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
			let this = self.get_mut();
			if let Some(promise) = &mut this.write_promise {
				match Pin::new(promise).poll(cx) {
					Pending => Pending,
					Ready(Ok(_)) => {
						this.write_promise = None;
						Ready(Ok(()))
					}
					Ready(Err(err)) => {
						this.write_promise = None;
						let msg = err.as_string().unwrap_or_else(|| format!("{:?}", err));
						Ready(Err(Error::new(Other, msg)))
					}
				}
			} else {
				Ready(Ok(()))
			}
		}

		fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
			let this = self.get_mut();
			this.inner.close().ignore();
			let mut js_future = JsFuture::from(this.inner.closed());
			match Pin::new(&mut js_future).poll(cx) {
				Pending => Pending,
				Ready(Ok(_)) => Ready(Ok(())),
				Ready(Err(err)) => {
					let msg = err.as_string().unwrap_or_else(|| format!("{:?}", err));
					Ready(Err(Error::new(Other, msg)))
				}
			}
		}
	}
}

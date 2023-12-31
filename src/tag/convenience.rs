#![allow(dead_code)]

use std::{
	any,
  boxed,
  error,
  fmt,
  marker,
  borrow::{Borrow, BorrowMut},
  ops::{self, Deref, DerefMut},
};

use super::tagged::{deserialize, serialize, Deserialize, Serialize};

/// Convenience wrapper around [std::boxed::Box<T>](std::boxed::Box) that automatically uses `serde_traitobject` for (de)serialization.
#[derive(Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Box<T: ?Sized>(boxed::Box<T>);
impl<T> Box<T> {
	/// Create a new Box wrapper
	pub fn new(t: T) -> Self {
		Self(boxed::Box::new(t))
	}
}
impl<T: ?Sized> Box<T> {
	/// Convert to a regular `std::Boxed::Box<T>`. Coherence rules prevent currently prevent `impl Into<std::boxed::Box<T>> for Box<T>`.
	pub fn into_box(self) -> boxed::Box<T> {
		self.0
	}
}
impl Box<dyn Any> {
	/// Convert into a `std::boxed::Box<dyn std::any::Any>`.
	pub fn into_any(self) -> boxed::Box<dyn any::Any> {
		self.0.into_any()
	}
}
impl Box<dyn Any + Send> {
	/// Convert into a `std::boxed::Box<dyn std::any::Any + Send>`.
	pub fn into_any_send(self) -> boxed::Box<dyn any::Any + Send> {
		unsafe {
			boxed::Box::from_raw(boxed::Box::into_raw(<Box<dyn Any>>::into_any(self)) as *mut _)
		}
	}
}
impl Box<dyn Any + Sync> {
	/// Convert into a `std::boxed::Box<dyn std::any::Any + Sync>`.
	pub fn into_any_sync(self) -> boxed::Box<dyn any::Any + Sync> {
		unsafe {
			boxed::Box::from_raw(boxed::Box::into_raw(<Box<dyn Any>>::into_any(self)) as *mut _)
		}
	}
}
impl Box<dyn Any + Send + Sync> {
	/// Convert into a `std::boxed::Box<dyn std::any::Any + Send + Sync>`.
	pub fn into_any_send_sync(self) -> boxed::Box<dyn any::Any + Send + Sync> {
		unsafe {
			boxed::Box::from_raw(boxed::Box::into_raw(<Box<dyn Any>>::into_any(self)) as *mut _)
		}
	}
}
impl dyn Any + Send {
	/// Convert into a `std::boxed::Box<dyn std::any::Any + Send>`.
	pub fn into_any_send(self: boxed::Box<Self>) -> boxed::Box<dyn any::Any + Send> {
		<Box<dyn Any + Send>>::into_any_send(Box(self))
	}
}
impl dyn Any + Sync {
	/// Convert into a `std::boxed::Box<dyn std::any::Any + Sync>`.
	pub fn into_any_sync(self: boxed::Box<Self>) -> boxed::Box<dyn any::Any + Sync> {
		<Box<dyn Any + Sync>>::into_any_sync(Box(self))
	}
}
impl dyn Any + Send + Sync {
	/// Convert into a `std::boxed::Box<dyn std::any::Any + Send + Sync>`.
	pub fn into_any_send_sync(self: boxed::Box<Self>) -> boxed::Box<dyn any::Any + Send + Sync> {
		<Box<dyn Any + Send + Sync>>::into_any_send_sync(Box(self))
	}
}
impl<T: ?Sized + marker::Unsize<U>, U: ?Sized> ops::CoerceUnsized<Box<U>> for Box<T> {}
impl<T: ?Sized> Deref for Box<T> {
	type Target = boxed::Box<T>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl<T: ?Sized> DerefMut for Box<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
impl<T: ?Sized> AsRef<boxed::Box<T>> for Box<T> {
	fn as_ref(&self) -> &boxed::Box<T> {
		&self.0
	}
}
impl<T: ?Sized> AsMut<boxed::Box<T>> for Box<T> {
	fn as_mut(&mut self) -> &mut boxed::Box<T> {
		&mut self.0
	}
}
impl<T: ?Sized> AsRef<T> for Box<T> {
	fn as_ref(&self) -> &T {
		&self.0
	}
}
impl<T: ?Sized> AsMut<T> for Box<T> {
	fn as_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: ?Sized> Borrow<T> for Box<T> {
	fn borrow(&self) -> &T {
		&self.0
	}
}
impl<T: ?Sized> BorrowMut<T> for Box<T> {
	fn borrow_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: ?Sized> From<boxed::Box<T>> for Box<T> {
	fn from(t: boxed::Box<T>) -> Self {
		Self(t)
	}
}
// impl<T: ?Sized> Into<boxed::Box<T>> for Box<T> {
// 	fn into(self) -> boxed::Box<T> {
// 		self.0
// 	}
// }
impl<T> From<T> for Box<T> {
	fn from(t: T) -> Self {
		Self(boxed::Box::new(t))
	}
}
impl<T: error::Error> error::Error for Box<T> {
	#[allow(deprecated)]
	fn description(&self) -> &str {
		error::Error::description(&**self)
	}
	#[allow(deprecated)]
	fn cause(&self) -> Option<&dyn error::Error> {
		error::Error::cause(&**self)
	}
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		error::Error::source(&**self)
	}
}
impl<T: fmt::Debug + ?Sized> fmt::Debug for Box<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		self.0.fmt(f)
	}
}
impl<T: fmt::Display + ?Sized> fmt::Display for Box<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		self.0.fmt(f)
	}
}
impl<T: Serialize + ?Sized + 'static> serde::ser::Serialize for Box<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serialize(&self.0, serializer)
	}
}

impl<'de, T: Deserialize + ?Sized + 'static> serde::de::Deserialize<'de> for Box<T> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserialize(deserializer).map(Self)
	}
}

/// A convenience trait implemented on all (de)serializable implementors of [`std::any::Any`].
///
/// It can be made into a trait object which is then (de)serializable.
///
/// # Example
/// ```
/// extern crate serde_json;
/// extern crate serde_traitobject as s;
///
/// use std::any::Any;
///
/// let erased: s::Box<dyn s::Any> = s::Box::new(String::from("hi there"));
///
/// let serialized = serde_json::to_string(&erased).unwrap();
/// let deserialized: s::Box<dyn s::Any> = serde_json::from_str(&serialized).unwrap();
///
/// let downcast: Box<String> = Box::<dyn Any>::downcast(deserialized.into_any()).unwrap();
///
/// println!("{}!", downcast);
/// # assert_eq!(format!("{}!", downcast), "hi there!");
/// // hi there!
/// ```
pub trait Any: any::Any + Serialize + Deserialize {
	/// Convert to a `&std::any::Any`.
	fn as_any(&self) -> &dyn any::Any;
	/// Convert to a `&mut std::any::Any`.
	fn as_any_mut(&mut self) -> &mut dyn any::Any;
	/// Convert to a `std::boxed::Box<dyn std::any::Any>`.
	fn into_any(self: boxed::Box<Self>) -> boxed::Box<dyn any::Any>;
}
impl<T> Any for T
where
	T: any::Any + Serialize + Deserialize,
{
	fn as_any(&self) -> &dyn any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn any::Any {
		self
	}
	fn into_any(self: boxed::Box<Self>) -> boxed::Box<dyn any::Any> {
		self
	}
}

impl AsRef<Self> for dyn Any {
	fn as_ref(&self) -> &Self {
		self
	}
}
impl AsRef<Self> for dyn Any + Send {
	fn as_ref(&self) -> &Self {
		self
	}
}

impl serde::ser::Serialize for dyn Any {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serialize(self, serializer)
	}
}

impl<'de> serde::de::Deserialize<'de> for boxed::Box<dyn Any + 'static> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		<Box<dyn Any + 'static>>::deserialize(deserializer).map(|x| x.0)
	}
}

impl serde::ser::Serialize for dyn Any + Send {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serialize(self, serializer)
	}
}

impl<'de> serde::de::Deserialize<'de> for boxed::Box<dyn Any + Send + 'static> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		<Box<dyn Any + Send + 'static>>::deserialize(deserializer).map(|x| x.0)
	}
}

/// A convenience trait implemented on all (de)serializable implementors of [`std::error::Error`].
///
/// It can be made into a trait object which is then (de)serializable.
///
/// # Example
/// ```
/// # extern crate serde;
/// #[macro_use] extern crate serde_derive;
/// extern crate serde_json;
/// extern crate serde_traitobject as s;
///
/// use std::fmt;
///
/// #[derive(Serialize,Deserialize,Debug)]
/// struct MyError(String);
/// impl fmt::Display for MyError {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         write!(f, "{}", self.0)
///     }
/// }
/// impl std::error::Error for MyError {}
///
/// fn fallible() -> Result<(),s::Box<dyn s::Error>> {
///     Err(Box::new(MyError(String::from("boxed error"))) as Box<dyn s::Error>)?
/// }
///
/// let serialized = serde_json::to_string(&fallible()).unwrap();
/// let deserialized: Result<(),s::Box<dyn s::Error>> = serde_json::from_str(&serialized).unwrap();
///
/// println!("{:?}", deserialized);
/// # assert_eq!(format!("{:?}", deserialized), "Err(MyError(\"boxed error\"))");
/// // Err(MyError("boxed error"))
/// ```
pub trait Error: error::Error + Serialize + Deserialize {}
impl<T: ?Sized> Error for T where T: error::Error + Serialize + Deserialize {}

impl<'a> AsRef<Self> for dyn Error + 'a {
	fn as_ref(&self) -> &Self {
		self
	}
}
impl<'a> AsRef<Self> for dyn Error + Send + 'a {
	fn as_ref(&self) -> &Self {
		self
	}
}

impl<'a, E: error::Error + Serialize + Deserialize + 'a> From<E> for Box<dyn Error + 'a> {
	fn from(err: E) -> Self {
		Box::new(err)
	}
}
impl<'a, E: error::Error + Serialize + Deserialize + 'a> From<E> for boxed::Box<dyn Error + 'a> {
	fn from(err: E) -> Self {
		boxed::Box::new(err)
	}
}

impl serde::ser::Serialize for dyn Error {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serialize(self, serializer)
	}
}

impl<'de> serde::de::Deserialize<'de> for boxed::Box<dyn Error + 'static> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		<Box<dyn Error + 'static>>::deserialize(deserializer).map(|x| x.0)
	}
}

impl serde::ser::Serialize for dyn Error + Send {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serialize(self, serializer)
	}
}

impl<'de> serde::de::Deserialize<'de> for boxed::Box<dyn Error + Send + 'static> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		<Box<dyn Error + Send + 'static>>::deserialize(deserializer).map(|x| x.0)
	}
}

/// A convenience trait implemented on all (de)serializable implementors of [`std::fmt::Display`].
///
/// It can be made into a trait object which is then (de)serializable.
///
/// # Example
/// ```
/// # extern crate serde;
/// #[macro_use] extern crate serde_derive;
/// extern crate serde_json;
/// extern crate serde_traitobject as s;
///
/// fn message() -> s::Box<dyn s::Display> {
///     s::Box::new(String::from("boxed displayable"))
/// }
///
/// let serialized = serde_json::to_string(&message()).unwrap();
/// let deserialized: s::Box<dyn s::Display> = serde_json::from_str(&serialized).unwrap();
///
/// println!("{}", deserialized);
/// # assert_eq!(format!("{}", deserialized), "boxed displayable");
/// // boxed displayable
/// ```
pub trait Display: fmt::Display + Serialize + Deserialize {}
impl<T: ?Sized> Display for T where T: fmt::Display + Serialize + Deserialize {}

impl<'a> AsRef<Self> for dyn Display + 'a {
	fn as_ref(&self) -> &Self {
		self
	}
}
impl<'a> AsRef<Self> for dyn Display + Send + 'a {
	fn as_ref(&self) -> &Self {
		self
	}
}

impl serde::ser::Serialize for dyn Display {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serialize(self, serializer)
	}
}

impl<'de> serde::de::Deserialize<'de> for boxed::Box<dyn Display + 'static> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		<Box<dyn Display + 'static>>::deserialize(deserializer).map(|x| x.0)
	}
}

impl serde::ser::Serialize for dyn Display + Send {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serialize(self, serializer)
	}
}

impl<'de> serde::de::Deserialize<'de> for boxed::Box<dyn Display + Send + 'static> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		<Box<dyn Display + Send + 'static>>::deserialize(deserializer).map(|x| x.0)
	}
}

/// A convenience trait implemented on all (de)serializable implementors of [`std::fmt::Debug`].
///
/// It can be made into a trait object which is then (de)serializable.
///
/// # Example
/// ```
/// # extern crate serde;
/// #[macro_use] extern crate serde_derive;
/// extern crate serde_json;
/// extern crate serde_traitobject as s;
///
/// fn debug() -> s::Box<dyn s::Debug> {
///     s::Box::new(String::from("boxed debuggable"))
/// }
///
/// let serialized = serde_json::to_string(&debug()).unwrap();
/// let deserialized: s::Box<dyn s::Debug> = serde_json::from_str(&serialized).unwrap();
///
/// println!("{:?}", deserialized);
/// # assert_eq!(format!("{:?}", deserialized), "\"boxed debuggable\"");
/// // "boxed debuggable"
/// ```
pub trait Debug: fmt::Debug + Serialize + Deserialize {}
impl<T: ?Sized> Debug for T where T: fmt::Debug + Serialize + Deserialize {}

impl<'a> AsRef<Self> for dyn Debug + 'a {
	fn as_ref(&self) -> &Self {
		self
	}
}
impl<'a> AsRef<Self> for dyn Debug + Send + 'a {
	fn as_ref(&self) -> &Self {
		self
	}
}

impl serde::ser::Serialize for dyn Debug {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serialize(self, serializer)
	}
}

impl<'de> serde::de::Deserialize<'de> for boxed::Box<dyn Debug + 'static> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		<Box<dyn Debug + 'static>>::deserialize(deserializer).map(|x| x.0)
	}
}

impl serde::ser::Serialize for dyn Debug + Send {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serialize(self, serializer)
	}
}

impl<'de> serde::de::Deserialize<'de> for boxed::Box<dyn Debug + Send + 'static> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		<Box<dyn Debug + Send + 'static>>::deserialize(deserializer).map(|x| x.0)
	}
}

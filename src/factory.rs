use std::marker::PhantomData;
use serde::{Serialize, Deserialize};
use erased_serde::serialize_trait_object;

pub trait Resource: 'static + erased_serde::Serialize + Send + Sync + std::fmt::Debug {
}

impl<'erased> ::erased_serde::__private::serde::Serialize for dyn Resource + 'erased {
  fn serialize<S>(
    &self,
    serializer: S,
  ) -> ::erased_serde::__private::Result<S::Ok, S::Error>
  where
    S: ::erased_serde::__private::serde::Serializer,
  {
    fn __check_erased_serialize_supertrait<__T>()
    where
      __T: ?::erased_serde::__private::Sized + Resource,
    {
      ::erased_serde::__private::require_erased_serialize_impl::<__T>();
    }
    //::erased_serde::serialize(self, serializer)
    //serializer.serialize_i32(1)
    use serde::ser::SerializeStruct;
    let mut state = serializer.serialize_struct("Test", 1)?;
    state.serialize_field("value", &self)?;
    state.end()
  }
}
//serialize_trait_object!(Resource);
/*
impl<'erased> Serialize for dyn Resource + 'erased {
  fn serialize<S>(&self, serializer: S) -> ::erased_serde::__private::Result<S::Ok, S::Error>
  where
      S: erased_serde::__private::serde::Serializer,
  {
    ::erased_serde::serialize(self, serializer)
  }
}
*/

pub trait ErasedResource: 'static + std::fmt::Debug + Send + Sync {
}
//serialize_trait_object!(ErasedResource);


#[derive(Debug)]
pub struct ResourceImpl<T>
  where T: std::fmt::Debug,
{
  #[allow(unused)]
  name: &'static str,
  _phantom: PhantomData<*const T>,
}

unsafe impl<T> Send for ResourceImpl<T> where T: std::fmt::Debug {}
unsafe impl<T> Sync for ResourceImpl<T> where T: std::fmt::Debug {}

impl<T> ResourceImpl<T> where T: std::fmt::Debug {
  pub fn new(name: &'static str) -> Self {
    Self {
      name,
      _phantom: PhantomData,
    }
  }
}

impl<T> ErasedResource for ResourceImpl<T>
  where
    //T: std::fmt::Debug + Serialize + for<'a> Deserialize<'a> + 'static
    T: std::fmt::Debug + erased_serde::Serialize + 'static
{
}

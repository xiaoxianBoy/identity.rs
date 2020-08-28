#[macro_export]
macro_rules! object {
  () => {
    $crate::common::Object::default()
  };
  ($($key:ident : $value:expr),* $(,)*) => {
    {
      let mut object = ::std::collections::HashMap::new();

      $(
        object.insert(
          stringify!($key).to_string(),
          $crate::common::Value::from($value),
        );
      )*

      $crate::common::Object::from(object)
    }
  };
}

macro_rules! impl_builder_setter {
  ($fn:ident, $field:ident, Option<$ty:ty>) => {
    pub fn $fn(mut self, value: impl Into<$ty>) -> Self {
      self.$field = Some(value.into());
      self
    }
  };
  ($fn:ident, $field:ident, Vec<$ty:ty>) => {
    pub fn $fn(mut self, value: impl Into<$ty>) -> Self {
      self.$field.push(value.into());
      self
    }
  };
  ($fn:ident, $field:ident, $ty:ty) => {
    pub fn $fn(mut self, value: impl Into<$ty>) -> Self {
      self.$field = value.into();
      self
    }
  };
}

macro_rules! impl_builder_try_setter {
  ($fn:ident, $field:ident, Option<$ty:ty>) => {
    pub fn $fn<T>(mut self, value: T) -> ::anyhow::Result<Self>
    where
      $ty: ::std::convert::TryFrom<T, Error = Error>,
    {
      use ::std::convert::TryFrom;
      <$ty>::try_from(value)
        .map(|value| {
          self.$field = Some(value);
          self
        })
        .map_err(Into::into)
    }
  };
  ($fn:ident, $field:ident, Vec<$ty:ty>) => {
    pub fn $fn<T>(mut self, value: T) -> ::anyhow::Result<Self>
    where
      $ty: ::std::convert::TryFrom<T, Error = Error>,
    {
      use ::std::convert::TryFrom;
      <$ty>::try_from(value)
        .map(|value| {
          self.$field.push(value);
          self
        })
        .map_err(Into::into)
    }
  };
  ($fn:ident, $field:ident, $ty:ty) => {
    pub fn $fn<T>(mut self, value: T) -> ::anyhow::Result<Self>
    where
      $ty: ::std::convert::TryFrom<T, Error = Error>,
    {
      use ::std::convert::TryFrom;
      <$ty>::try_from(value)
        .map(|value| {
          self.$field = value;
          self
        })
        .map_err(Into::into)
    }
  };
}

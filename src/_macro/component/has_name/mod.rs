#[macro_export]
macro_rules! has_name {
  ($data: expr, $entity: expr, $name: expr) => {{
    #[allow(unused_imports)]
    use $crate::ecs::component::*;
    $data
      .has_name
      .insert($entity, HasName($name.into()))
      .expect("Unable to insert has-name for entity!");
  }};
}

#[macro_export]
macro_rules! get_name {
  ($data: expr, $entity: expr) => {{
    $data.has_name.get($entity).map(|has_name| &has_name.0)
  }};
}

#[macro_export]
macro_rules! get_lc_name {
  ($data: expr, $entity: expr) => {{
    get_name!($data, $entity).cloned().map(|name| name.to_lowercase())
  }};
}

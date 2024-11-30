use std::any::{Any, TypeId};

pub fn struct_is_type<T: 'static>(struct_any: &dyn Any) -> bool {
    TypeId::of::<T>() == struct_any.type_id()
}

pub fn struct_downcast_ref<T: 'static>(struct_any: Box<dyn Any>) -> Result<Box<T>,Box<dyn Any>> {
    struct_any.downcast::<T>()
}
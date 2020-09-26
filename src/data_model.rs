use glib::subclass;
use glib::subclass::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use glib::bitflags::_core::marker::PhantomData;

pub trait DataModelDescription {
    const NAME: &'static str;
    fn get_properties() -> &'static [subclass::Property<'static>];
}

pub struct DataModelPrivate<T : 'static + DataModelDescription> {
    d: PhantomData<T>,
    data: RefCell<HashMap<String, glib::Value>>,
}

impl<T: 'static + DataModelDescription> ObjectSubclass for DataModelPrivate<T> {
    const NAME: &'static str = T::NAME;
    type ParentType = glib::Object;
    type Instance = subclass::simple::InstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;
    glib_object_subclass!();
    fn class_init(klass: &mut Self::Class) {
        klass.install_properties(T::get_properties());
    }

    fn new() -> Self {
        Self {
            d: PhantomData::default(),
            data: RefCell::new(HashMap::new())
        }
    }
}

impl<T: 'static + DataModelDescription> ObjectImpl for DataModelPrivate<T> {
    glib_object_impl!();
    fn set_property(&self, _obj: &glib::Object, id: usize, value: &glib::Value) {
        let prop = &T::get_properties()[id];
        self.data.borrow_mut().insert(prop.0.to_string(), value.to_owned());
    }

    fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
        let prop = &T::get_properties()[id];
        self.data.borrow().get(prop.0).map(|v| v.to_owned()).ok_or(())
    }
}

#[macro_export]
macro_rules! data_model {
    ($x:tt) => {
        glib_wrapper! {
                pub struct $x(Object<subclass::simple::InstanceStruct<DataModelPrivate<$x>>, subclass::simple::ClassStruct<DataModelPrivate<$x>>, RowDataClass>);
                match fn {
                    get_type => || DataModelPrivate::<$x>::get_type().to_glib(),
                }
        }

        impl $x {
            pub fn new(properties: &[(&str, &dyn ToValue)]) -> $x {
                glib::Object::new(Self::static_type(), properties)
                    .expect("Failed to create row data")
                    .downcast()
                    .expect("Created row data is of wrong type")
            }
        }
    };
}
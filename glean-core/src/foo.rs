use once_cell::sync::OnceCell;
use std::sync::Mutex;

use crate::error::Result;

static FOO_BUILDER: OnceCell<Mutex<Option<Builder>>> = OnceCell::new();
pub fn setup_foo_builder(builder: Option<Builder>) -> Result<()> {
    if builder.is_some() {
        // If we are starting a new builder, we have to drop the current FOO.
        setup_foo(None).expect("Unable to drop FOO");
    }

    if FOO_BUILDER.get().is_none() {
        if FOO_BUILDER.set(Mutex::new(builder)).is_err() {
            log::error!(
                "Global Builder object is initialized already. This probably happened concurrently."
            )
        }
    } else {
        let mut lock = FOO_BUILDER.get().unwrap().lock().unwrap();
        *lock = builder;

    }

    Ok(())
}

pub fn global_foo_builder() -> Option<&'static Mutex<Option<Builder>>> {
    FOO_BUILDER.get()
}

static FOO: OnceCell<Mutex<Option<Foo>>> = OnceCell::new();
pub fn setup_foo(foo: Option<Foo>) -> Result<()> {
    if foo.is_some() {
        // If we are starting a new builder, we have to drop the current FOO.
       setup_foo(None).expect("Unable to drop FOO");
    }

    if FOO.get().is_none() {
        if FOO.set(Mutex::new(foo)).is_err() {
            log::error!(
                "Global Foo object is initialized already. This probably happened concurrently."
            )
        }
    } else {
        let mut lock = FOO.get().unwrap().lock().unwrap();
        *lock = foo;

    }

    Ok(())
}

pub fn global_foo() -> Option<&'static Mutex<Option<Foo>>> {
    FOO.get()
}

#[derive(Default)]
pub struct Builder {
    pub bar: Option<String>
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bar<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.bar = Some(value.into());
        log::info!("Set builder `bar` to {:?}", self.bar.as_ref());
        self
    }

    pub fn build(&self) -> Foo {
        Foo {
            bar: self.bar.as_ref().unwrap_or(&String::from("WHATUP")).to_owned()
        }
    }
}

pub struct Foo {
    bar: String
}

impl Foo {
    pub fn bar(&self) -> &str {
        &self.bar
    }

    pub fn set_bar<S: Into<String>>(&mut self, value: S) {
        self.bar = value.into();
    }
}

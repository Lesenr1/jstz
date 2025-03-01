use boa_engine::{
    object::builtins::{
        JsArray, JsArrayBuffer, JsDataView, JsDate, JsFloat32Array, JsFloat64Array,
        JsFunction, JsGenerator, JsInt16Array, JsInt32Array, JsInt8Array, JsMap,
        JsMapIterator, JsPromise, JsProxy, JsRegExp, JsSet, JsSetIterator, JsTypedArray,
        JsUint16Array, JsUint32Array, JsUint8Array,
    },
    Context, JsBigInt, JsObject, JsString, JsSymbol, JsValue,
};

pub use boa_engine::value::*;

pub trait IntoJs {
    /// This function converts a Rust value into a JavaScript value.
    fn into_js(self, context: &mut Context<'_>) -> JsValue;
}

macro_rules! impl_into_js_from_into {
    ($($T: ty), *) => {
        $(
            impl IntoJs for $T {
                #[inline]
                fn into_js(self, _context: &mut Context<'_>) -> JsValue {
                    self.into()
                }
            }
        )*
    };
}

impl_into_js_from_into!(
    JsArray,
    JsArrayBuffer,
    JsBigInt,
    JsDataView,
    JsDate,
    JsFloat32Array,
    JsFloat64Array,
    JsFunction,
    JsGenerator,
    JsInt16Array,
    JsInt32Array,
    JsInt8Array,
    JsMap,
    JsMapIterator,
    JsObject,
    JsPromise,
    JsProxy,
    JsRegExp,
    JsSet,
    JsSetIterator,
    JsSymbol,
    JsTypedArray,
    JsUint16Array,
    JsUint32Array,
    JsUint8Array,
    JsString,
    Numeric,
    (),
    char,
    f32,
    f64,
    i16,
    i32,
    i64,
    i8,
    u16,
    u32,
    u64,
    u8,
    usize,
    String
);

impl<T> IntoJs for Option<T>
where
    T: IntoJs,
{
    fn into_js(self, context: &mut Context<'_>) -> JsValue {
        match self {
            Some(value) => value.into_js(context),
            None => JsValue::null(),
        }
    }
}

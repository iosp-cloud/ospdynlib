extern crate libloading;
use std::{any::Any, collections::HashMap};
use std::mem::transmute;

pub type Result<T> = std::result::Result<T, libloading::Error>;

pub type AnyType = dyn Any;

#[derive(Debug)]
/**
 * 
 * 
 * 
 */
pub struct DynLibrary<'lib> {
    // 动态库名称
    filename:&'lib str,
    // 指向动态库
    dynlib: libloading::Library,
    // get_service_component,get_service_plugin函数
    fn_map: HashMap<&'lib str, libloading::Symbol<'lib, *const ()>>,
    // // 指向ServiceComponent的指针
    // sc_map: HashMap<&'lib str, &'lib Box<*const AnyType >>,
    // // 指向ServicePlugin的指针
    // pl_map: HashMap<&'lib str, &'lib Box<*const AnyType >>,
}
/**
 * 
 * 
 * 
 */
impl<'lib> DynLibrary<'lib> {
    /**
     * 
     * 
     * 
     */
    pub fn new(filename: &'lib str) -> Result<DynLibrary<'lib>> {
        Ok(DynLibrary {
            filename: filename,
            dynlib: libloading::Library::new(filename)?,
            fn_map: HashMap::new(),
            // sc_map: HashMap::new(),
            // pl_map: HashMap::new(),
        })
    }
    /**
     * 
     * 
     * 
     */
    pub unsafe fn get<T>(&'lib  self, symbol: &'lib str) -> Result<&'lib libloading::Symbol<'lib, T>> {
        // match self.fn_map.get::<str>(symbol) {
        //     Some(func) => {
        //         let func = transmute::<&libloading::Symbol<*const()>, &libloading::Symbol<T>>(func);
        //         return Ok(func);
        //     },
        //     None => {}
        // };
        let func = self.dynlib.get(symbol.as_bytes())?;
        // self.fn_map.insert(symbol, func);
        // let func = self.fn_map.get::<str>(symbol).unwrap();
        let func = transmute::<&libloading::Symbol<*const()>, &libloading::Symbol<T>>(&func);
        return Ok(func);
    }
}
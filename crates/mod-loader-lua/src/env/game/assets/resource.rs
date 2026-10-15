use std::rc::Rc;

use kfc::reflection::LookupKey;
use mlua::{AnyUserData, ObjectLike, UserData};

use crate::{alias::TypeHandle, env::{AppState, ResourceInfo, game::Type}, lua::{LuaValue, MethodArgs}};

use super::value::convert_lua_to_json;

pub struct Resource {
    info: Rc<ResourceInfo>,
}

impl Resource {

    pub fn new(info: Rc<ResourceInfo>) -> Self {
        Self {
            info,
        }
    }

    pub fn info(&self) -> &Rc<ResourceInfo> {
        &self.info
    }

}

impl UserData for Resource {

    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("guid", |_, this| Ok(this.info.resource_id.to_string()));
        fields.add_field_method_get("type", |lua, this| {
            let app_state = lua.app_data_ref::<AppState>().unwrap();
            let hash = this.info.resource_id.type_hash();
            let index = app_state.type_registry()
                .get_by_hash(LookupKey::Qualified(hash))
                .map(|t| t.index);

            if let Some(index) = index {
                app_state.get_type(lua, index)
            } else {
                Ok(Some(LuaValue::Nil))
            }
        });
        fields.add_field_method_get("part", |_, this| Ok(this.info.resource_id.part_index()));
        fields.add_field_method_get("data", |lua, this| {
            this.info.get_lua_value(lua)
        });
        fields.add_field_method_set("data", |lua, this, value: LuaValue| {
            this.info.set_lua_value(value, lua)
        });
    }

    // TODO
    // fn add_methods<M: mlua::prelude::LuaUserDataMethods<Self>>(methods: &mut M) {

    //     methods.add_meta_function("__to_json", |lua, args: MethodArgs| {
    //         let this = args.this::<&Self>()?;
    //         let luathis = args.this::<LuaValue>()?;

    //         let pretty = args.try_get::<bool>(0)?.unwrap_or(false);

    //         let typehandle = luathis.as_userdata().unwrap().call_method::<Type>("type", ())?;

    //         Ok(LuaValue::String(lua.create_string(convert_lua_to_json(luathis, typehandle.handle(), pretty)?)?))
    //     });
    // }
}

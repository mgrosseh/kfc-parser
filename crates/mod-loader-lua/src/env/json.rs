use std::any;

use mlua::{MetaMethod};

use crate::lua::{FunctionArgs};


// pub fn create(
//     lua: &mlua::Lua,
// ) -> mlua::Result<mlua::Table> {
//     let table = lua.create_table()?;

//     add_function(lua, &table, "encode", lua_encode)?;

//     Ok(table)
// }


pub fn lua_tojson(
    lua: &mlua::Lua,
    args: FunctionArgs,
) -> mlua::Result<mlua::String> {
    let obj = args.get::<mlua::Value>(0)?;
    let pretty = args.try_get::<bool>(1)?.unwrap_or(false);
    match obj {
        mlua::Value::Nil => lua.create_string("null"),
        mlua::Value::Boolean(v) => lua.create_string(v.to_string()),
        mlua::Value::LightUserData(light_user_data) => todo!(),
        mlua::Value::Integer(v) => lua.create_string(v.to_string()),
        mlua::Value::Number(v) => lua.create_string(v.to_string()),
        mlua::Value::String(v) => lua.create_string(serde_json::to_string(v).map_err(mlua::Error::external)?),
        mlua::Value::Table(table) => todo!(),
        mlua::Value::Function(function) => todo!(),
        mlua::Value::Thread(thread) => todo!(),
        mlua::Value::UserData(any_user_data) => {
            let metatable = any_user_data.metatable()?;
            if metatable.contains("__to_json")? {
                let x = metatable.get::<mlua::Function>("__to_json")?;
                x.call::<mlua::String>((any_user_data, pretty))
            }
            else {

                let type_name = metatable.get::<mlua::String>(MetaMethod::Type)?;

                Err(mlua::Error::external(format!("Cannot access __to_json in {}", type_name.to_str()?)))
            }
        },
        mlua::Value::Error(error) => todo!(),
        mlua::Value::Other(value_ref) => todo!(),
    }
    //lua.create_string(&buffer)
}

use mlua::{ExternalError, ExternalResult, MultiValue, Table, TableExt, Value};
use tokio::runtime::Handle;

use super::slim_lua;
use crate::LOADED;

pub async fn entry(name: &str, args: Vec<String>) -> mlua::Result<()> {
	LOADED.ensure(name).await.into_lua_err()?;

	let name = name.to_owned();
	tokio::task::spawn_blocking(move || {
		let lua = slim_lua()?;
		let plugin: Table = if let Some(b) = LOADED.read().get(&name) {
			lua.load(b).call(())?
		} else {
			return Err("unloaded plugin".into_lua_err());
		};
		Handle::current().block_on(plugin.call_async_method(
			"entry",
			MultiValue::from_iter(args.iter().map(|v| Value::String(lua.create_string(v).unwrap()))),
		))
	})
	.await
	.into_lua_err()?
}

use std::fs;

use common::GlobalCommand;
use mlua::{FromLua, Function, Lua, UserData};

#[derive(Debug, Clone, FromLua)]
pub struct CommandLoader {
    pub commands: Vec<GlobalCommand>,
}

impl CommandLoader {
    pub fn new() -> Self {
        CommandLoader { commands: vec![] }
    }
    pub fn push_command(&mut self, value: GlobalCommand) {
        self.commands.push(value);
    }

    pub fn get_command_names(&self) -> Vec<String> {
        self.commands.iter().map(|cmd| cmd.name.clone()).collect()
    }
}

impl UserData for CommandLoader {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("register_global_command", |_, this, name: String| {
            this.push_command(GlobalCommand { name });
            Ok(())
        });
        methods.add_method("get_command_names", |_, this, ()| {
            Ok(this.get_command_names())
        });
    }
}

pub fn register_all_commands() -> Vec<GlobalCommand> {
    let lua = Lua::new();
    lua.globals()
        .set("launchpad", CommandLoader::new())
        .expect("failed to set extension handler");

    for file in fs::read_dir("./mods").unwrap(){
        let entry=file.unwrap();
        let src=fs::read_to_string(entry.path()).expect(&("failed to read lua file".to_owned()+entry.path().to_str().unwrap()));
        
        lua.load(src).exec().expect("Failed to execute mod");

        let handler:Function=lua.globals().get("handler").expect("Handler function not defined");
        let result:Vec<String>=handler.call(23).expect("Failed to execute function");

        println!("{:#?}",result);
    }
    /*lua.load(
        r#"
        launchpad:register_global_command("command1")
        launchpad:register_global_command("command2")
    "#,
    )
    .exec()
    .expect("Failed to execute mod");*/

    let commands: CommandLoader = lua
        .globals()
        .get("launchpad")
        .expect("Failed to retrieve commands after mod execution");
    
    commands.commands
}

pub use mlua;
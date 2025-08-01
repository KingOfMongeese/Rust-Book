pub trait Plugin {
    fn run(&self);

    fn name(&self) -> String;

    fn hello(&self) -> String {
        format!("hello from {}", self.name())
    }
}

pub struct BasicPlugin {
    pub name: String,
}

impl Plugin for BasicPlugin {
    fn run(&self) {
        println!("Running plugin {}", self.name);
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct PluginWithMetadata {
    pub name: String,
    pub author: String,
    pub version: String,
}

impl Plugin for PluginWithMetadata {
    fn run(&self) {
        println!("Running plugin {}@{} by {}", self.name, self.version, self.author);
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

// need to pass trait objects using dyn since size isnt known
pub fn run_all_plugins(plugins: &[Box<dyn Plugin>])
{
    plugins.iter().for_each(|plugin| plugin.run());
}


pub fn get_all_plugin_names(plugins: &[Box<dyn Plugin>]) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    plugins.iter().for_each(|plugin| names.push(plugin.name()));
    names
}

pub fn hello_from_all_plugins(plugins: &[Box<dyn Plugin>]) {
    for plugin in plugins {
        println!("{}", plugin.hello());
    }
}
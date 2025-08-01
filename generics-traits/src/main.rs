#![warn(clippy::pedantic)]

mod traits;
mod generics;
use crate::traits::{BasicPlugin, PluginWithMetadata, Plugin};

fn main() {
    let plugin_a = BasicPlugin {
        name: "Plugin A".to_string(),
    };

    let plugin_b = PluginWithMetadata {
        name: "Plugin B".to_string(),
        author: "KingOfMongeese".to_string(),
        version: "v1.2.3".to_string(),
    };

    let plugins: Vec<Box<dyn Plugin>> = vec![Box::new(plugin_a), Box::new(plugin_b)];

    traits::hello_from_all_plugins(&plugins);
    traits::run_all_plugins(&plugins);
    for name in traits::get_all_plugin_names(&plugins) {
        println!("{name}");
    }

    // generic lests us use the same func for diff types, as long as they support are req trait: PartialOrd
    println!("--------------------------------------------");
    let list1 = vec![1, 2, 45, 50];
    let list2 = vec![5, 60];

    println!("Biggest 1 or 2: {}", generics::biggest(&list1, &list2));

    println!("--------------------------------------------");
    let list1 = vec![1.0, 2.0, 45.0, 81.0];
    let list2 = vec![5.0, 60.0];

    println!("Biggest 1 or 2: {}", generics::biggest(&list1, &list2));


}

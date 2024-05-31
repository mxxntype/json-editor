use std::collections::HashMap;
use std::mem;

#[derive(Default)]
pub struct JsonEditor {
    /// The JSON key that's currently being edited.
    pub key_input: String,
    /// The JSON value that's currently being edited.
    pub value_input: String,
    /// The representation of our key and value pairs.
    pub pairs: HashMap<String, String>,
    /// The current screen the user is looking at.
    /// Determines what is being rendered.
    pub current_screen: ActiveScreen,
    /// Whether the user is editing a JSON key or value.
    /// Optional, because a user might not be editing anything.
    pub editing_mode: Option<EditingMode>,
}

impl JsonEditor {
    /// This function is called when the user saves a key-value pair in the editor.
    ///
    /// It adds the two stored variables to the key-value `HashMap`,
    /// and resets the status of all of the editing variables.
    pub fn save_kv_pair(&mut self) {
        self.pairs.insert(
            mem::take(&mut self.key_input),
            mem::take(&mut self.value_input),
        );
        self.editing_mode = None;
    }

    /// Check if something is currently being edited, and if so, swap between editing the Key and Value fields.
    pub fn toggle_editing_mode(&mut self) {
        if let Some(editing_mode) = &self.editing_mode {
            match editing_mode {
                EditingMode::Key => self.editing_mode = Some(EditingMode::Value),
                EditingMode::Value => self.editing_mode = Some(EditingMode::Key),
            };
        } else {
            self.editing_mode = Some(EditingMode::Key);
        }
    }

    // /// Another convenience function to print out the serialized json from all of our key-value pairs.
    // pub fn print_json(&self) -> serde_json::Result<()> {
    //     let output = serde_json::to_string(&self.pairs)?;
    //     println!("{}", output);
    //     Ok(())
    // }
}

#[derive(PartialEq, Eq, Default)]
pub enum ActiveScreen {
    #[default]
    Main,
    Editing,
    Exiting,
}

#[derive(PartialEq, Eq, Default)]
pub enum EditingMode {
    #[default]
    Key,
    Value,
}

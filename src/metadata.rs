/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
/**
class Parameter {
  constructor(name) {
    /* Parameter information */
    // The parameter path using dot notation
    this.name = name;
    // The parameter description
    this.description = '';
    // The parameter value
    this.value = undefined;
    // The parameter type
    this.type = '';

    /* Extra metadata about the parameter */
    // The modifiers applied to the parameter as an array of strings
    this.modifiers = [];
    // The section the parameter belongs to
    this.section = '';

    /* Properties to manage tool behaviour for this parameter */
    // Skips the check of the parameter
    this.validate = true;
    // Whether to render the paramter into the README
    this.readme = true;
    // Whether to render the paramter into the schema
    this.schema = true;
  }

  // Extra parameters won't be checked but will be rendered on the README
  set extra(extra) {
    if (extra) {
      this.validate = false;
      this.readme = true;
    }
  }

  get extra() {
    return (!this.validate && this.readme);
  }

  set skip(skip) {
    if (skip) {
      this.validate = false;
      this.readme = false;
    } else {
      this.validate = true;
      this.readme = true;
    }
  }

  get skip() {
    return (!this.validate && !this.readme);
  }
}
**/

// Param defines a chart values
pub struct Param {
    name: String,
    param_type: Option<String>,
    value: Option<String>,
    descr: Option<String>,
    modifiers: Vec<String>,
}

impl Param {
    pub(crate) fn new(name: String, modifiers: Vec<String>, descr: Option<String>) -> Param {
        Param{
            name,
            param_type: None,
            value: None,
            modifiers,
            descr,
        }
    }
}

// Section defines a param section
pub struct Section {
    name: String,
    descr: Vec<String>,
    params: Vec<&Param>,
}

impl Section {
    pub fn new(name: String) -> Section {
        Section {
            name,
            descr: Vec::new(),
            params:  Vec::new(),
        }
    }

    pub fn add_param(mut self, param: &Param) {
        self.params.push(param)
    }

    pub fn add_descr(mut self, line: String) {
        self.descr.push(line);
    }
}

// Metadata defines the general metadata defined in a chart values file
pub struct Metadata {
    sections: Vec<&Section>,
    params: Vec<&Param>
}

impl Metadata {
    pub(crate) fn new() -> Metadata {
        Metadata{
            sections: Vec::new(),
            params: Vec::new(),
        }
    }

    pub fn add_section(mut self, section: &Section) {
        self.sections.push(section)
    }

    pub fn add_param(mut self, param: &Param) {
        self.params.push(param)
    }
}

#[derive(Clone, Default, Debug)]
pub struct TrueFalseValue {
    value: Option<bool>,
    value_default: bool,
}
impl TrueFalseValue {
    pub(crate) fn get_value(&self) -> &bool {
        match &self.value {
            Some(v) => v,
            None => &self.value_default,
        }
    }

    pub(crate) fn get_value_string(&self) -> &str {
        if !*self.get_value() {
            "f"
        } else {
            "t"
        }
    }

    pub(crate) fn set_value(&mut self, value: bool) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.set_value(value.into() != "f")
    }

    pub(crate) fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub(crate) fn _get_hash_string(&self) -> &str {
        if self.has_value() {
            return self.get_value_string();
        }
        "empty!!"
    }
}

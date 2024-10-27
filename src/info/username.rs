use super::ModuleVar;

pub struct UserNameVar<'a> {
    pub name: &'a str,
}

impl ModuleVar<!> for UserNameVar<'_> {
    fn new() -> Self {
        Self { name: "username" }
    }

    fn value(&mut self, _cfg: Option<&!>) -> String {
        let users = sysinfo::Users::new_with_refreshed_list();
        users.list()[0].name().to_string()
    }
}

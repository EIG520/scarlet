pub trait TuneContainer : Default {
    fn get_refmut(&mut self, name: &str) -> &mut i32;
    fn list_options(&self);
    fn list_wf_json_config(&self);
}

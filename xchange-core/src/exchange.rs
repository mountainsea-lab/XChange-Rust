/// Exchange trait
pub trait Exchange {
    const USE_SANDBOX: &'static str = "Use_Sandbox";

    type Spec;
    type Meta;
    type Instrument;

    fn get_exchange_specification(&self) -> &Self::Spec;
    fn get_exchange_meta_data(&self) -> &Self::Meta;
    fn get_exchange_instruments(&self) -> Vec<Self::Instrument>;
    fn get_default_exchange_specification(&self) -> &Self::Spec;
    fn apply_specification(&mut self, spec: Self::Spec);
}

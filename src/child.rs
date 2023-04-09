use num::One;
use crate::pval::PVal;

#[derive(Default, Debug, Clone)]
pub struct Child {
    pub key: PVal,
    pub val: PVal,
    pub ptr: Option<&'static Child>,
    pub occupied: i8
}

impl Child {

}
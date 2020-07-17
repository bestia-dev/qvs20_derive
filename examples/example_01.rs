// example_01.rs
// is used by test_01 to cargo expand
// then is compared to saved expand_01.txt

use qvs20_derive::{Qvs20Row,Qvs20Table};

// clear; cargo expand --example example_01

#[derive(Qvs20Table)]
pub struct CouDenTable (Vec<CouDenRow>);

#[derive(Qvs20Row)]
#[Qvs20TableName = "cou_den5"]
#[Qvs20Description = "example with country population density"]
pub struct CouDenRow {
    pub country: String,
    pub density: String,
}

fn main(){

}
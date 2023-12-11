mod SpecialN;
mod SpecialNSpin;
mod SpecialNEnd;

pub fn install() {
    SpecialN::install();
    SpecialNSpin::install();
    SpecialNEnd::install(); 
}
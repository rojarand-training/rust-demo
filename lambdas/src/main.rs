struct SavingsCalculator{
    incomes: f32,
}
impl SavingsCalculator{
    fn calculate(&self, tax_calculation: fn(f32)->f32)->f32{
        let tax = tax_calculation(self.incomes);
        self.incomes-tax
    }
}

fn main() {
    let savings_calculator = SavingsCalculator{incomes: 100.0};
    //inline impl
    let savings_in_pl = savings_calculator.calculate(|inc |{ inc*0.23});
    println!("savings in pl: {:}",savings_in_pl);

    let us_tax_calculation = |inc:f32| -> f32 {inc*0.17};
    let savings_in_us = savings_calculator.calculate(us_tax_calculation);
    println!("savings in us: {:}",savings_in_us);

}

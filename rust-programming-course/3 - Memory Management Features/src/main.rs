#[derive(Debug)]
struct ArrayProcessor<'a> {
    data: &'a [i32],
}

impl<'a> ArrayProcessor<'a> {
    fn update_data(&mut self, arr: &'a [i32]) -> &[i32] {
        let previous_data = self.data;
        self.data = arr;
        previous_data
    }

}

fn main() {
    let mut some_data = ArrayProcessor { data: &[4, 5, 6] };
    let curious_data = &[5, 6, 7];

    {
        let prev_data = some_data.update_data(curious_data);
        println!("Prev data: {:?}", prev_data);
        println!("Some data: {:?}", some_data);
    }
}

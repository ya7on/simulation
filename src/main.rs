use simulation::ai::neural::Neural;

fn main() {
    let input_layers_num = 2;
    let layers_num = 15;
    let layers_size = 5;
    let output_layers_num = 5;

    let neural = Neural::new(input_layers_num, layers_num, layers_size, output_layers_num);
    println!("{:?}", neural.execute(vec![1.0, 0.0]));
    let neural = neural.mutate();
    println!("{:?}", neural.execute(vec![1.0, 0.0]));
}

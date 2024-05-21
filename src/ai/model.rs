use std::path::Path;

use tensorflow::{Graph, SavedModelBundle, SessionOptions, SessionRunArgs, Tensor};

use crate::output;

pub struct JaModel{

}

impl JaModel{
    fn train<P: AsRef<Path>>(model_path: P){
        let train_input_name: &str = "training_input";
        let train_target_name: &str = "training_target";
        let pred_input_name: &str = "inputs";

        let train_output_name: &str = "output_0";
        let pred_output_name: &str = "output_0";

        let input_tensor: Tensor<f32> = Tensor::new(&[1, 2]).with_values(&[1.0, 1.0]).unwrap();
        let target_tensor: Tensor<f32> = Tensor::new(&[1,1]).with_values(&[2.0]).unwrap();

        let mut graph = Graph::new();

        let bundle = SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, model_path).expect("Can't load saved model");

        let session = &bundle.session;

        let signature_train = bundle.meta_graph_def().get_signature("train").unwrap();

        let input_train = signature_train.get_input(train_input_name).unwrap();
        let target_train = signature_train.get_input(train_input_name).unwrap();

        let output_train = signature_train.get_output(train_output_name).unwrap();

        let input_op_train = graph.operation_by_name_required(&input_train.name().name).unwrap();
        let target_op_train = graph.operation_by_name_required(&target_train.name().name).unwrap();

        let output_op_train = graph.operation_by_name_required(&output_train.name().name).unwrap();

        let mut args = SessionRunArgs::new();

        args.add_feed(&input_op_train, 0, &input_tensor);
        args.add_feed(&target_op_train, 0, &target_tensor);

        let mut out = args.request_fetch(&output_op_train, 0);

        session.run(&mut args). expect("Failed to perform calculations");

        let loss: f32 = args.fetch(out).unwrap()[0];

        println!("Loss: {:?}", loss);



    }
}

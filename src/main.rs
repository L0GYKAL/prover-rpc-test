use std::error::Error;
use madara_prover_rpc_client::services::starknet_prover::execute_and_prove;
use madara_prover_rpc_client::services::starknet_prover::starknet_prover_proto::starknet_prover_client::StarknetProverClient;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    prove_cairo_program().await;
}

pub async fn prove_cairo_program() -> () { //Result<(), Box<dyn Error>>
    let mut client = StarknetProverClient::connect("grpc://[::1]:8080").await.unwrap();
    let compiled_program = std::fs::read("fibo/fibonacci_compiled.json").unwrap();
    //let params = std::fs::read("example_cairo_program/fibonacci/target/dev/fibonacci.starknet_artifacts.json").unwrap();

    let programs = vec![compiled_program];
    let pies = vec![];
    let split_proof = false;

    println!("execute and prove");
    let proof = execute_and_prove(&mut client, programs, pies, split_proof).await.unwrap();
    println!("Proof: {}", proof.proof_hex);

}

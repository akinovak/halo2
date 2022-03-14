mod circuit;

use halo2_proofs::{
    dev::MockProver,
    pairing::bn256::Fr as Fp
};
use circuit::{Circuit, NUM_OF_T1_COLUMNS, T1_SIZE, NUM_OF_T2_COLUMNS, T2_SIZE};


fn main() {
    let one = Some(Fp::from(2 as u64));
    let table1: [[Option<Fp>; T1_SIZE]; NUM_OF_T1_COLUMNS] = 
        [
            [one, one, one],
            [one, one, one]
        ];
    let table2: [[Option<Fp>; T2_SIZE]; NUM_OF_T2_COLUMNS] = 
        [
            [one, one, one],
            [one, one, one]
        ];

    let circuit = Circuit {
        table1,
        table2
    };

    let one = one.unwrap();

    let pub_inputs = vec![
        vec![one, one, one, one, one, one, one, one, one, one, one, one],
        vec![one, one, one], 
        vec![one, one, one],
        vec![one, one, one],
        vec![one, one, one],
    ];

    let k = 10;
    let prover = MockProver::run(k, &circuit, pub_inputs).unwrap();
    assert_eq!(prover.verify(), Ok(()));
}
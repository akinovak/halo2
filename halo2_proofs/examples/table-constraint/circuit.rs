use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{Layouter, SimpleFloorPlanner},
    plonk::{Advice, Instance, Column, ConstraintSystem, Error},
    plonk,
};
use std::convert::TryInto;

pub const NUM_OF_T1_COLUMNS: usize = 2;
pub const T1_SIZE: usize = 3;
pub const NUM_OF_T2_COLUMNS: usize = 2;
pub const T2_SIZE: usize = 3;
 //there is 1 + size for each table num of instances
pub const NUM_OF_INSTANCE_COLUMNS: usize = 1 + NUM_OF_T1_COLUMNS + NUM_OF_T2_COLUMNS;


#[derive(Clone, Debug)]
pub struct Config {
    advice: Column<Advice>,
    flat_instance: Column<Instance>,
    t1_instance: [Column<Instance>; NUM_OF_T1_COLUMNS],
    t2_instance: [Column<Instance>; NUM_OF_T2_COLUMNS],
}


#[derive(Clone, Debug, Default)]
pub struct Circuit<F: FieldExt> {
    pub table1: [[Option<F>; T1_SIZE]; NUM_OF_T1_COLUMNS],
    pub table2: [[Option<F>; T2_SIZE]; NUM_OF_T2_COLUMNS]
}


impl<F: FieldExt> plonk::Circuit<F> for Circuit<F> {
    type Config = Config;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {

        let advice = meta.advice_column();
        meta.enable_equality(advice);

        let mut instance: Vec<Column<Instance>> = vec![];
        for _i in 0..NUM_OF_INSTANCE_COLUMNS {
            let i = meta.instance_column();
            meta.enable_equality(i);
            instance.push(i);

        }

        Config {
            advice,
            flat_instance: instance[0],
            t1_instance: instance[1..NUM_OF_T1_COLUMNS + 1].try_into().unwrap(),
            t2_instance: instance[NUM_OF_T1_COLUMNS + 1..].try_into().unwrap(),
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let config = config.clone();

        let mut row_cnt = 0;
        for (i_col, column) in self.table1.iter().enumerate() {
            for (i_row, value) in column.iter().enumerate() {

                let assigned = layouter.assign_region(
                            || "load t1",
                            |mut region| {
                                let assigned = region
                                    .assign_advice(
                                        || format!("r {}", row_cnt),
                                        config.advice,
                                        row_cnt,
                                || value.ok_or(Error::Synthesis),
                            )?;
                        Ok(assigned)
                    },
                )?;

                layouter.constrain_instance(assigned.cell(), config.flat_instance, row_cnt)?;
                layouter.constrain_instance(assigned.cell(), config.t1_instance[i_col], i_row)?;

                row_cnt += 1;
            }
        }

        for (i_col, column) in self.table2.iter().enumerate() {
            for (i_row, value) in column.iter().enumerate() {

                let assigned = layouter.assign_region(
                            || "load t1",
                            |mut region| {
                                let assigned = region
                                    .assign_advice(
                                        || format!("r {}", row_cnt),
                                        config.advice,
                                        row_cnt,
                                || value.ok_or(Error::Synthesis),
                            )?;
                        Ok(assigned)
                    },
                )?;

                layouter.constrain_instance(assigned.cell(), config.flat_instance, row_cnt)?;
                layouter.constrain_instance(assigned.cell(), config.t2_instance[i_col], i_row)?;

                row_cnt += 1;
            }
        }

        Ok(())
    }
}
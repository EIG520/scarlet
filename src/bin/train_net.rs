// This file is essentially copied from Bullet's examples

use bullet::{
    LocalSettings, TrainingSchedule, TrainingSteps,
    game::inputs::Chess768,
    lr,
    nn::optimiser::AdamW,
    trainer::save::SavedFormat,
    value::{ValueTrainerBuilder, loader},
    wdl,
};

const HIDDEN: usize = 512;
const SCALE: i32 = 400;
const QA: i16 = 255;
const QB: i16 = 64;

pub fn main() {
    let mut trainer = ValueTrainerBuilder::default()
        .dual_perspective()
        .optimiser(AdamW)
        .inputs(Chess768)
        .save_format(&[
            SavedFormat::id("l0w").round().quantise::<i16>(QA),
            SavedFormat::id("l0b").round().quantise::<i16>(QA),
            SavedFormat::id("l1w").round().quantise::<i16>(QB),
            SavedFormat::id("l1b").round().quantise::<i16>(QA * QB),
        ])
        .loss_fn(|output, target| output.sigmoid().squared_error(target))
        .build(|builder, stm_inputs, ntm_inputs| {
            // weights
            let l0 = builder.new_affine("l0", 768, HIDDEN);
            let l1 = builder.new_affine("l1", 2 * HIDDEN, 1);

            // inference
            let stm_hidden = l0.forward(stm_inputs).screlu();
            let ntm_hidden = l0.forward(ntm_inputs).screlu();
            let hidden_layer = stm_hidden.concat(ntm_hidden);
            l1.forward(hidden_layer)
        });

    let schedule = TrainingSchedule {
        net_id: "witch_512_3".to_string(),
        eval_scale: SCALE as f32,
        steps: TrainingSteps {
            batch_size: 16_384,
            batches_per_superbatch: 6104,
            start_superbatch: 1,
            end_superbatch: 400,
        },
        wdl_scheduler: wdl::LinearWDL {
            start: 0.2,
            end: 0.5,
        },
        lr_scheduler: lr::CosineDecayLR {
            initial_lr: 0.001,
            final_lr: 0.001 * 0.3f32.powi(5),
            final_superbatch: 400,
        },
        save_rate: 10,
    };

    let settings = LocalSettings {
        threads: 16,
        test_set: None,
        output_directory: "checkpoints",
        batch_queue_size: 64,
    };

    let data_loader = {
        use loader::viribinpack::{Filter, ViriBinpackLoader, ViriFilter};

        let file_path = r"train7.viri";
        let buffer_size_mb = 1024;
        let threads = 16;

        // The `viriformat` crate exposes a useful `Filter` of its own, but you can also
        // use a custom function like for SF binpacks with `ViriFilter::custom(function)`
        let mut internal_filter = Filter::default();
        internal_filter.max_eval = 25000;
        let filter = ViriFilter::Builtin(internal_filter);

        ViriBinpackLoader::new(file_path, buffer_size_mb, threads, filter)
    };

    trainer.run(&schedule, &settings, &data_loader);
}

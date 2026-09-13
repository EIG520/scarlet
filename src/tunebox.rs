use tune_macro::TuneContainer;
use tune_macro_derive::TuneContainer;

#[derive(Clone, Copy, TuneContainer)]
pub struct TuneBox {
    #[Tunable(default = 104, min = 0, max = 999999, step = 10)]
    pub rfp_depthmul: i32,
    #[Tunable(default = 79, min = 0, max = 999999, step = 10)]
    pub rfp_improving: i32,
    #[Tunable(default = 32, min = 0, max = 999999, step = 10)]
    pub time_num: i32,
    #[Tunable(default = 618, min = 0, max = 999999, step = 100)]
    pub inc_num: i32,
    #[Tunable(default = 130, min = 0, max = 999999, step = 50)]
    pub nmp_dmul: i32,
    #[Tunable(default = 287, min = 0, max = 999999, step = 100)]
    pub nmp_ddiv: i32,
    #[Tunable(default = 67, min = 0, max = 999999, step = 50)]
    pub ffp_dmul: i32,
    #[Tunable(default = 581, min = -999999, max = 999999, step = 100)]
    pub ffp_c: i32,
    #[Tunable(default = 536, min = 0, max = 999999, step = 1000)]
    pub singmarg_dmul: i32,
    #[Tunable(default = 578, min = 0, max = 999999, step = 200)]
    pub singd_dmul: i32,
    #[Tunable(default = 2535, min = -999999, max = 999999, step = 500)]
    pub lmr_c: i32,
    #[Tunable(default = 257, min = 0, max = 999999, step = 300)]
    pub lmr_num: i32,
    #[Tunable(default = 236, min = 0, max = 999999, step = 100)]
    pub histred_div: i32,
    #[Tunable(default = 2535, min = -999999, max = 999999, step = 500)]
    pub lmr_c_pv: i32,
    #[Tunable(default = 99, min = 0, max = 999999, step = 300)]
    pub lmr_num_pv: i32,
    #[Tunable(default = 96, min = 0, max = 999999, step = 100)]
    pub histred_div_pv: i32,
    #[Tunable(default = 1737, min = 0, max = 999999, step = 500)]
    pub hist_inc: i32,
    #[Tunable(default = 1916, min = 0, max = 999999, step = 500)]
    pub hist_inc_pv: i32,
    #[Tunable(default = 1058, min = 0, max = 999999, step = 500)]
    pub hist_dec: i32,
    #[Tunable(default = 1180, min = 0, max = 999999, step = 500)]
    pub hist_dec_pv: i32,
    #[Tunable(default = 1103, min = 0, max = 999999, step = 500)]
    pub hist_dec_quitact: i32,
    #[Tunable(default = 467, min = 0, max = 999999, step = 500)]
    pub hist_dec_pv_quitact: i32,
    #[Tunable(default = 881, min = 0, max = 999999, step = 500)]
    pub hist_inc_tact: i32,
    #[Tunable(default = 1617, min = 0, max = 999999, step = 500)]
    pub hist_inc_pv_tact: i32,
    #[Tunable(default = 2491, min = 0, max = 999999, step = 500)]
    pub hist_dec_ttact: i32,
    #[Tunable(default = 1065, min = 0, max = 999999, step = 500)]
    pub hist_dec_pv_ttact: i32,
    #[Tunable(default = 31, min = -999999, max = 999999, step = 20)]
    pub see_prune_margin: i32,
    #[Tunable(default = -19, min = -999999, max = 999999, step = 20)]
    pub see_sort_margin: i32,
}

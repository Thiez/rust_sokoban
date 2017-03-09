use libc::{size_t,c_int};

pub type BDD = u64;
pub type BDDSET = u64;
pub type BDDVAR = u32;

#[allow(dead_code)]
#[link(name = "sylvan")]
extern {
  pub static sylvan_true: BDD;
  pub static sylvan_true_nc: BDD;
  pub static sylvan_false: BDD;
  pub static sylvan_invalid: BDD;
  pub fn lace_init(workers: c_int, x: size_t, stacksize: size_t);
  pub fn sylvan_init(datasize: size_t, cachesize: size_t, granularity: c_int);
  pub fn sylvan_quit();
  pub fn sylvan_ithvar(var: BDDVAR) -> BDD;
  pub fn sylvan_nithvar(var: BDDVAR) -> BDD;
  pub fn sylvan_cube(variables: *const BDDVAR, count: size_t, cube: *mut u8) -> BDD;
  pub fn sylvan_bdd_to_nocomp(bdd: BDD) -> BDD;
  pub fn sylvan_var(bdd: BDD) -> BDDVAR;
  pub fn sylvan_low(bdd: BDD) -> BDD;
  pub fn sylvan_high(bdd: BDD) -> BDD;
  pub fn sylvan_not(bdd: BDD) -> BDD;
  pub fn sylvan_ite(a: BDD, b: BDD, c: BDD) -> BDD;

  pub fn sylvan_and(a: BDD, b: BDD) -> BDD;
  pub fn sylvan_xor(a: BDD, b: BDD) -> BDD;
  pub fn sylvan_or(a: BDD, b: BDD) -> BDD;
  pub fn sylvan_nand(a: BDD, b: BDD) -> BDD;
  pub fn sylvan_nor(a: BDD, b: BDD) -> BDD;
  pub fn sylvan_imp(a: BDD, b: BDD) -> BDD;
  pub fn sylvan_biimp(a: BDD, b: BDD) -> BDD;
  pub fn sylvan_diff(a: BDD, b: BDD) -> BDD;
  pub fn sylvan_less(a: BDD, b: BDD) -> BDD;
  pub fn sylvan_invimp(a: BDD, b: BDD) -> BDD;

  pub fn sylvan_relprods(a: BDD, b: BDD, vars: BDD) -> BDD;
  pub fn sylvan_relprods_reversed(a: BDD, b: BDD, vars: BDD) -> BDD;
  pub fn sylvar_relprod(a: BDD, b: BDD, vars: BDD) -> BDD;
  pub fn sylvan_substitute(a: BDD, vars: BDD) -> BDD;

  pub fn sylvan_constrain(a: BDD, b: BDD);
  pub fn sylvan_exists(a: BDD, variables: BDD);
  pub fn sylvan_forall(a: BDD, variables: BDD);

  pub fn sylvan_support(bdd: BDD) -> BDD;

  pub fn sylvan_ref(a: BDD) -> BDD;
  pub fn sylvan_deref(a: BDD);
  pub fn sylvan_count_refs() -> size_t;
  pub fn sylvan_gc();

  pub fn sylvan_gc_enable();
  pub fn sylvan_gc_disable();
  pub fn sylvan_reset_counters();
  pub fn sylvan_report_stast();

  pub fn sylvan_set_isempty(set: BDDSET) -> c_int;
  pub fn sylvan_set_var(set: BDDSET) -> BDDVAR;
  pub fn sylvan_set_empty() -> BDDSET;
  pub fn sylavn_set_add(set: BDDSET, level: BDDVAR) -> BDDSET;
  pub fn sylvan_set_addall(set: BDDSET, toadd: BDD) -> BDDSET;
  pub fn sylvan_set_remove(set: BDDSET, level: BDDVAR) -> BDDSET;
  pub fn sylvan_set_removeall(set: BDDSET, toremove: BDDSET) -> BDDSET;
  pub fn sylvan_set_in(set: BDDSET, level: BDDVAR) -> c_int;
  pub fn sylvan_set_next(set: BDDSET) -> BDDSET;
  pub fn sylvan_set_count(set: BDDSET) -> size_t;
  pub fn sylvan_set_toarray(set: BDDSET, arr: *mut BDDVAR);
  pub fn sylvan_set_fromarray(arr: *const BDDVAR, length: size_t);

  pub fn sylvan_makenode(level: BDDVAR, low: BDD, high: BDD) -> BDD;

  pub fn sylvan_printdot(bdd: BDD);
  pub fn sylvan_printdot_nocomp(bdd: BDD);
  pub fn sylvan_print(bdd: BDD);

  pub fn sylvan_satcount(bdd: BDD, variables: BDD) -> f64;
  pub fn sylvan_sat_one(bdd: BDD, variable: *const BDDVAR, count: size_t, str: *const u8) -> c_int;
  pub fn sylvan_pathcount(bdd: BDD) -> f64;

  pub fn sylvan_nodecount(a: BDD) -> size_t;

}

pub unsafe fn raw_init() {
  lace_init(1,100000,0);
  sylvan_init(26,22,2);
}




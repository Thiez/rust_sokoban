use raw::{BDD};
use raw::{sylvan_low,sylvan_high,sylvan_ite};
use raw::{sylvan_and,sylvan_xor,sylvan_or,sylvan_nand,
          sylvan_nor,sylvan_imp,sylvan_biimp,sylvan_diff,
          sylvan_less,sylvan_invimp};
use raw::{sylvan_not};
use raw::{sylvan_ithvar};

struct Bdd(BDD);

fn unpack(a: &Bdd, b: Bdd) -> (BDD,BDD) {
  let (Bdd(a),Bdd(b)) = (*a,b);
  (a,b)
}

impl Eq for Bdd {
  fn eq(&self, other: &Bdd) -> bool {
    let (&Bdd(a),&Bdd(b)) = (self,other);
    a == b
  }
}

impl Not<Bdd> for Bdd {
  fn not(&self) -> Bdd {
    let &Bdd(a) = self;
    unsafe {
      Bdd(sylvan_not(a))
    }
  }
}

impl BitAnd<Bdd,Bdd> for Bdd {
  fn bitand(&self, rhs: &Bdd) -> Bdd {
    let (a,b) = unpack(self,*rhs);
    unsafe {
      Bdd(sylvan_and(a,b))
    }
  }
}

impl BitOr<Bdd,Bdd> for Bdd {
  fn bitor(&self, rhs: &Bdd) -> Bdd {
    let (a,b) = unpack(self,*rhs);
    unsafe {
      Bdd(sylvan_or(a,b))
    }
  }
}

impl BitXor<Bdd,Bdd> for Bdd {
  fn bitxor(&self, rhs: &Bdd) -> Bdd {
    let (a,b) = unpack(self,*rhs);
    unsafe {
      Bdd(sylvan_xor(a,b))
    }
  }
}

#[allow(dead_code)]
impl Bdd {
  pub fn fromId(id: u32) -> Bdd {
    unsafe {
      Bdd(sylvan_ithvar(id))
    }
  }

  pub fn low(&self) -> Bdd {
    let &Bdd(a) = self;
    unsafe {
      Bdd(sylvan_low(a))
    }
  }

  pub fn high(&self) -> Bdd {
    let &Bdd(a) = self;
    unsafe {
      Bdd(sylvan_high(a))
    }
  }

  pub fn not(&self) -> Bdd {
    let &Bdd(a) = self;
    unsafe {
      Bdd(sylvan_not(a))
    }
  }

  pub fn ite(&self, bdd_if: Bdd, bdd_else: Bdd) -> Bdd {
    let (&Bdd(a),Bdd(b),Bdd(c)) = (self,bdd_if,bdd_else);
    unsafe {
      Bdd(sylvan_ite(a,b,c))
    }
  }

  pub fn and(&self, other: Bdd) -> Bdd {
    let (a,b) = unpack(self,other);
    unsafe {
      Bdd(sylvan_and(a,b))
    }
  }
  pub fn xor(&self, other: Bdd) -> Bdd {
    let (a,b) = unpack(self,other);
    unsafe {
      Bdd(sylvan_xor(a,b))
    }
  }
  pub fn or(&self, other: Bdd) -> Bdd {
    let (a,b) = unpack(self,other);
    unsafe {
      Bdd(sylvan_or(a,b))
    }
  }
  pub fn nand(&self, other: Bdd) -> Bdd {
    let (a,b) = unpack(self,other);
    unsafe {
      Bdd(sylvan_nand(a,b))
    }
  }
  pub fn nor(&self, other: Bdd) -> Bdd {
    let (a,b) = unpack(self,other);
    unsafe {
      Bdd(sylvan_nor(a,b))
    }
  }
  pub fn imp(&self, other: Bdd) -> Bdd {
    let (a,b) = unpack(self,other);
    unsafe {
      Bdd(sylvan_imp(a,b))
    }
  }
  pub fn biimp(&self, other: Bdd) -> Bdd {
    let (a,b) = unpack(self,other);
    unsafe {
      Bdd(sylvan_biimp(a,b))
    }
  }
  pub fn diff(&self, other: Bdd) -> Bdd {
    let (a,b) = unpack(self,other);
    unsafe {
      Bdd(sylvan_diff(a,b))
    }
  }
  pub fn less(&self, other: Bdd) -> Bdd {
    let (a,b) = unpack(self,other);
    unsafe {
      Bdd(sylvan_less(a,b))
    }
  }
  pub fn invimp(&self, other: Bdd) -> Bdd {
    let (a,b) = unpack(self,other);
    unsafe {
      Bdd(sylvan_invimp(a,b))
    }
  }
}

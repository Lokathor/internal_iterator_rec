use internal_iterator_rec::{
  InternalIterator, InternalIteratorRec, adhoc_internal_iterator_rec,
};

pub enum Statement {
  Plain,
  Number(i32),
  Loop(Loop),
  IfElse(Vec<Statement>, Vec<Statement>),
}

impl Statement {
  pub fn iter_all_loops_by_ref(
    &self,
  ) -> impl '_ + InternalIteratorRec<Item = &'_ Loop> {
    adhoc_internal_iterator_rec!(
      'r, self, |this: &'r Statement, yield_| -> &'r Loop {
        match this {
          Statement::Plain => (),
          Statement::Number(_) => (),
          Statement::Loop(loop_) => {
            loop_.iter_all_loops_by_ref().try_for_each_rec(yield_)?;
          }
          Statement::IfElse(if_body, else_body) => {
            for stmt in if_body.iter() {
              stmt.iter_all_loops_by_ref().try_for_each_rec(yield_)?;
            }
            for stmt in else_body.iter() {
              stmt.iter_all_loops_by_ref().try_for_each_rec(yield_)?;
            }
          }
        }
      }
    )
  }

  pub fn iter_all_numbers_by_mut(
    &mut self,
  ) -> impl '_ + InternalIteratorRec<Item = &'_ mut i32> {
    adhoc_internal_iterator_rec!(
      'r, self, |this: &'r mut Statement, yield_| -> &'r mut i32 {
        match this {
          Statement::Plain => (),
          Statement::Number(i) => {
            yield_(i)?;
          }
          Statement::Loop(loop_) => {
            loop_.iter_all_numbers_by_mut().try_for_each_rec(yield_)?;
          }
          Statement::IfElse(if_body, else_body) => {
            for stmt in if_body.iter_mut() {
              stmt.iter_all_numbers_by_mut().try_for_each_rec(yield_)?;
            }
            for stmt in else_body.iter_mut() {
              stmt.iter_all_numbers_by_mut().try_for_each_rec(yield_)?;
            }
          }
        }
      }
    )
  }
}

pub struct Loop {
  name: String,
  body: Vec<Statement>,
}

impl Loop {
  // A type can even pass *itself* in when it's iterating by reference
  pub fn iter_all_loops_by_ref(
    &self,
  ) -> impl '_ + InternalIteratorRec<Item = &'_ Loop> {
    adhoc_internal_iterator_rec!(
      'r, self, |this: &'r Loop, yield_| -> &'r Loop {
        yield_(this)?;
        for stmt in this.body.iter() {
          stmt.iter_all_loops_by_ref().try_for_each_rec(yield_)?;
        }
      }
    )
  }

  // Types cannot pass themselves into the iteration by mutable reference
  // because of lifetime issues, but they can iterate mutably over other data
  // they might hold.
  pub fn iter_all_numbers_by_mut(
    &mut self,
  ) -> impl '_ + InternalIteratorRec<Item = &'_ mut i32> {
    adhoc_internal_iterator_rec!(
      'r, self, |this: &'r mut Loop, yield_| -> &'r mut i32 {
        for stmt in this.body.iter_mut() {
          stmt.iter_all_numbers_by_mut().try_for_each_rec(yield_)?;
        }
      }
    )
  }
}

#[test]
fn run_the_sample_code() {
  let mut s = Statement::Loop(Loop {
    name: String::from("one"),
    body: vec![
      Statement::Number(5),
      Statement::Loop(Loop {
        name: String::from("two"),
        body: vec![
          //
        ],
      }),
      Statement::Plain,
      Statement::Number(7),
      Statement::Loop(Loop {
        name: String::from("three"),
        body: vec![
          //
        ],
      }),
    ],
  });

  let mut names = Vec::new();
  s.iter_all_loops_by_ref().for_each(|loop_ref| {
    names.push(loop_ref.name.clone());
  });
  assert_eq!(vec!["one", "two", "three"], names);

  let mut numbers = Vec::new();
  s.iter_all_numbers_by_mut().for_each(|i32_mut| {
    numbers.push(*i32_mut);
  });
  assert_eq!(vec![5, 7], numbers);
}

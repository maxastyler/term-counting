extern crate nalgebra as na;
extern crate rayon;

use rayon::prelude::*;
use std::fmt;

struct Term {
    factor: u64,
    pi: u64,
    ds: na::DVector<u64>,
    es: na::DVector<u64>,
    deltas: na::DMatrix<u64>,
    terminal: bool,
}

impl Term {
    fn create_initial(a: usize, b: usize, c: usize, n: usize) -> Term {
        let mut ds_vec = vec![2; a];
        ds_vec.extend(vec![4; b]);
        ds_vec.extend(vec![6; c]);
        Term {
            factor: 1,
            pi: n as u64,
            ds: na::DVector::from_iterator(a+b+c, ds_vec),
            es: na::DVector::zeros(a+b+c),
            deltas: na::DMatrix::zeros(a+b+c, a+b+c),
            terminal: false,
        }
    }

    fn iterate(&self) -> Vec<Term> {
        if self.terminal { vec!(Term{
            factor: self.factor,
            pi: self.pi,
            ds: self.ds.clone(),
            es: self.es.clone(),
            deltas: self.deltas.clone(),
            terminal: true
        }) }
        else {
            if self.ds.iter().all(|&x| x==0u64) && self.pi == 0{
                vec!( Term {
                        factor: self.factor,
                        pi: self.pi,
                        ds: self.ds.clone(),
                        es: self.es.clone(),
                        deltas: self.deltas.clone(),
                        terminal: true} )
            }
            else {
                let mut new_terms = vec!();
                for (d_i, d_v) in self.ds.iter().enumerate() {
                    if *d_v > 0 {
                        if self.pi > 0 {
                            new_terms.push(self.take_derivative(d_i, None));
                        }
                        for (e_i, e_v) in self.es.iter().enumerate() {
                            if *e_v > 0 {
                                new_terms.push(self.take_derivative(d_i, Some(e_i)));
                            }
                        }
                    }
                }
                new_terms
            }
        }
    }

    fn take_derivative(&self, d_index: usize, maybe_es: Option<usize>) -> Term {
        match maybe_es {
            None => {
                let mut ds = self.ds.clone();
                ds[d_index] -= 1;
                let mut es = self.es.clone();
                es[d_index] += 1;
                Term{
                    factor: self.factor * self.pi,
                    pi: self.pi - 1,
                    ds: ds,
                    es: es,
                    deltas: self.deltas.clone(),
                    terminal: false,
                }
            },
            Some(i) => {
                let mut ds = self.ds.clone();
                ds[d_index] -= 1;
                let mut es = self.es.clone();
                es[i] -= 1;
                let mut deltas = self.deltas.clone();
                unsafe {
                *deltas.get_unchecked_mut(i, d_index) += 1;
                }
                Term{
                    factor: self.factor * self.es[i],
                    pi: self.pi,
                    ds: ds,
                    es: es,
                    deltas: deltas,
                    terminal: false,
                }
            }
        }
    }
}

impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Factor: {}\nPis: {}\nDS: {}ES: {}Deltas: {}Terminal: {}",
               self.factor, self.pi, self.ds, self.es, self.deltas, self.terminal
            )
    }
}

fn iterate_terms(terms: Vec<Term>) -> Vec<Term> {
    let mut new_terms: Vec<Term>= vec!();
    let mut a: Vec<_> = terms.par_iter().map(|ref x| x.iterate()).collect();
    for i in a.iter_mut() { new_terms.append(i); }
    new_terms
}
 
fn iterate_until_finished(init_terms: Vec<Term>) -> Vec<Term> {
    let mut terms = init_terms;
    loop{
        if terms.iter().all(|ref x| x.terminal) { break }
        terms = iterate_terms(terms);
    }
    terms
}

fn main() {
    let a = vec!(Term::create_initial(0, 0, 1, 5));
    let a = iterate_until_finished(a);
    //println!("{:?}", a);
    println!("{}", a.iter().map(|x| x.factor).sum::<u64>());
}

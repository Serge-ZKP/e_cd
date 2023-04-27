mod fr;
use ark_ff::Field;
// We'll use a field associated with the BLS12-381 pairing-friendly
// group for this example.
use crate::fr::Fr;

use ark_std::{One, UniformRand};




pub fn poly_e_cd_fr (c: Vec<Fr>, d: Vec<Fr>) -> Vec<Fr> {

    if c.len() == d.len() {
        let n = c.len();

        //Two vector just containig zero elements
        let mut c_tilde: Vec<Fr> = vec![Fr::from(1);n];
        let mut d_tilde: Vec<Fr> = vec![Fr::from(1);n];


        /*$\widetilde{c}_0 = c_0$
        For $i$ in range$(1,\; N-1)$:
        $\widetilde{c}_i := \widetilde{c}_{i-1} \cdot c_i$
         */

        c_tilde[0] = c[0];
        d_tilde[0] = d[0];


        for i in 1..n {
            c_tilde[i] = c_tilde[i - 1] * c[i];
            d_tilde[i] = d_tilde[i - 1] * d[i];
        }

        let mut d_hat : Vec<Fr> = vec![Fr::from(1);n];

        d_hat[ n - 1 ] = d_tilde[ n - 1 ].inverse().unwrap();

        //let mut e : Vec<Fr> = vec![Fr::from(1);n];

        for i in 1..n {
            //e[ n - i ]  = c_tilde[n-i] * d_hat [ n - i];
            d_hat[ n - 1 - i ] = d_hat[ n - i ]  * d[ n - i ];
        }


        let mut e: Vec<Fr> = vec![];
        for i in 0..n {
            e.push(c_tilde[i] * d_hat[i]);
        }
        return e;
    }
    else {
        println!("Dimenssions missmatch");
        let e:Vec<Fr> = vec![];
        return e;
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e_cd() {
        use std::time::Instant;
        //use ark_ff::Field;
        use ark_std::UniformRand;
        //use crate::fr::Fr;

        let mut rng = ark_std::test_rng();


        let mut c:Vec<Fr> = vec![];
        let mut d:Vec<Fr> = vec![];
        for _ in 1..3000000{
            c.push(Fr::rand(&mut rng));
            d.push(Fr::rand(&mut rng));
        }

        let now = Instant::now();
        let result = poly_e_cd_fr(c, d);
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        assert!(result.len() > 0);
    }
}

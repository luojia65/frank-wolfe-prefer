use frank_wolfe_prefer::{
    linalg::{MatBuf, EuclideanNorm, Norm}, 
    synthetic, 
    Omega
};

const M: usize = 500;
const N: usize = 40;

struct Local {
    ystar_i: MatBuf<f32>,
    yi_t1: MatBuf<f32>,
    omega_i: MatBuf<bool>
}

// impl Local {
//     pub fn update(&mut self, v: MatBuf<f32>, lambda_prime: f32, max_t: usize, t: usize, l: f32) -> MatBuf<f32> {
//         let ai_t1 = self.omega_i.p_omega(&self.yi_t1 - &self.ystar_i, 0.0);
//         let ui = &(&ai_t1 * &v) * (1.0 / lambda_prime);
//         let mut v = v; 
//         v.transpose();
//         let m = &self.yi_t1 * (1.0 - 1.0 / max_t as f32) - 
//             &ui * (1.0 /*?*/ / max_t as f32) * v;
//         let yi_t = self.omega_i.pi_l_omega(m, l, 1.0);
//         let ai_t = self.omega_i.p_omega(&yi_t - &self.ystar_i, 0.0);
//         if t < max_t {
//             self.yi_t1 = yi_t;
//             let mut ai_tt = ai_t.clone();
//             ai_tt.transpose();
//             return ai_t * ai_tt;
//         } else {
//             println!("Prediction: {:?}", yi_t);
//             return MatBuf::new_filled(0.0, [N, N])
//         }
//     }
// }

fn main() {
    let y_star_0: MatBuf<f32> = synthetic::data(M, N);
    let rate = 0.3;
    let omega = synthetic::omega(M, N, rate);
    let y_star = omega.p_omega(y_star_0, 0.0);
    let mut l = 0.0;
    let mut locals = Vec::new();
    for i in 0..M {
        let norm = EuclideanNorm::norm(&y_star[i]);
        l = f32::max(l, norm);
        locals.push(Local {
            ystar_i: MatBuf::from(y_star[i].to_vec()),
            yi_t1: MatBuf::new_filled(0.0, [N, 1]),
            omega_i: MatBuf::from(omega[i].to_vec()),
        })
    }
    println!("Parameter L = {}", l);
    let max_t = 10;
    let delta = 1e-6;
    let epsilon = 0.1;
    let beta = 10.0;
    let sigma = l * l * f32::sqrt(64.0 * max_t as f32 * f32::log10(1.0 / delta)) / epsilon;
    println!("Parameter sigma: {}", sigma);
    let mut lambda = 0.0;
    let mut v = MatBuf::new_filled(0.0, [1, N]);
    for t in 0..max_t {
        let mut w = MatBuf::new_filled(0.0, [N, N]);
        let lambda_prime = lambda + f32::sqrt(sigma * f32::log10(N as f32 / beta)) 
            * f32::powf(N as f32, 0.25);
        // for local in locals.iter_mut() {
        //     // w = w + local.update(v.clone(), lambda_prime, max_t, t, l);
        // }
        
    }
}

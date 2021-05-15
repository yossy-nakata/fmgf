/// ガウシアンフィルタ
pub struct Gaussian {}

impl Gaussian {

    /// 一次元ガウシアンフィルタを適用します。
    pub fn gaussian1d(array: &[f64], sigma: f64) -> Vec<f64> {
        let truncate = 4.0f64;
        let lw: isize = (truncate * sigma + 0.5f64) as isize;
        let weight = Self::karnel1d(sigma, lw);
        Self::correlate1d(array, &weight)
    }

    /// 一次元ガウス型カーネル。
    pub fn karnel1d(sigma: f64, radius: isize) -> Vec<f64> {
        let sigma2 = sigma * sigma;
        let x = -radius..radius + 1;
        let mut phi_x = Vec::<f64>::new();
        let mut total = 0f64;
         
        for n in x {
            let x_ = n as f64;
            let temp = ((-0.5 / sigma2) * (x_ * x_)).exp();
            total += temp;
            phi_x.push(temp);
        }
         
        for item in &mut phi_x {
            *item /= total;
        }
        phi_x
    }
    /// 一次元の畳み込み処理
    pub fn correlate1d(input: &[f64], weight: &[f64]) -> Vec<f64> {
        let mut rslt = Vec::<f64>::new();
        let sz = input.len();
        if sz == 0 {
            return rslt;
        }

        let side = (weight.len() / 2) as usize;
        let mut work = input.to_vec();
        for _ in 0..side {
            work.insert(0, input[0]);
            work.push(input[sz - 1]);
        }

        let ubound = work.len();
        for i in side..ubound - side {
            let mut smoothed = 0f64;
            for j in 0..weight.len() {
                let idx = i + j - side;
                smoothed += weight[j] * work[idx];
            }
            rslt.push(smoothed);
        }

        rslt
    }
}

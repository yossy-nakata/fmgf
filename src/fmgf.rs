use crate::{gf::Gaussian,matrix::Matrix};

/// 高速Ｍ推定ガウシアンフィルタ(Fast M-estimation based Gaussian Filter).
/// References:
///        Journal of the Japan Society for Precision Engineering Vol.76 (2010) No.6 P684-688
///        A Proposal of Robust Gaussian Filter by Using Fast M-Estimation Method
///        http://doi.org/10.2493/jjspe.76.684
pub struct Fmgf {}

impl Fmgf {
    
    /// 本フィルタの適用　
    pub fn fmgf(input: &[f64], sigma: f64, width_m:usize) -> Vec<f64> {
        // ｘ，ｙに分ける。
        let x: Vec<_> = (0u16..input.len() as u16).map(f64::from).collect();
        let mut y = input.to_vec();

        // ガウシアンフィルタを適用する。
        let yg = Gaussian::gaussian1d(&y, sigma);
        
        // 残差をとる。
        for i in 0..y.len() {
            y[i] -= yg[i];
        }

        // 残差の分布をｚ軸として離散化し累積する。
        let ybin = Self::grid(&y, width_m,5.0);
        let row_w = x.len();
        let col_h = ybin.len();
        let mut z = Matrix::new(col_h,row_w);
        let grid = Self::digitize(&y, &ybin,);
        for i in 0..x.len(){
            z.update(grid[i],i,1.0);
        }

        // 時間軸方向はガウス分布。
        for i in 0..col_h{
            let row =z.get_row(i).unwrap();
            z.set_row(i,&Gaussian::gaussian1d(&row,sigma));
        }

        // ｚ軸方向に矩形フィルタ（移動平均）を３回適用して、二次Ｂスプラインの制御点を得る。
        for i in 0..row_w{
            let col = z.get_col(i).unwrap();
            let sma1 = Self::sma(&col,width_m);
            let sma2 = Self::sma(&sma1,width_m);
            let sma3 = Self::sma(&sma2,width_m);
            z.set_col(i,&sma3);
        }
        
        // 各時間軸での制御点の最大値ｑを算出。
        let mut zmax=Vec::<usize>::new();
        for i in 0..row_w{
            let col=z.get_col(i).unwrap();
            let max_i=Self::argmax(&col).unwrap();
            zmax.push(max_i);
        }
        // 最大値ｑの周辺のｑ−１、ｑ、ｑ+１を用い解析的にピークを求める。
        let mut rslt=Vec::<f64>::new();
        for i in 0..row_w{
             let y0=ybin[zmax[i]];
             let ym1=ybin[zmax[i]-1];
             let yp1=ybin[zmax[i]+1];
             let z0=z.at(zmax[i],i);
             let zm1=z.at(zmax[i]-1,i);
             let zp1=z.at(zmax[i]+1,i);
             let t = (zm1-z0) / (zm1-2.0*z0+zp1);
             let ans = yg[i] + ((1.0-t).powi(2))*ym1 + (2.0*t*(1.0-t))*y0 + (t.powi(2))*yp1;
            rslt.push(ans);
        }

        rslt
    }

    /// デジタイズする。
    pub fn digitize(input: &[f64], bin: &[f64]) -> Vec<usize> {
        let mut rslts = Vec::<usize>::new();
        for v in input {   
            let mut rslt = 0;
            for b in bin{
                if *b > *v { 
                    break;
                }
                rslt += 1;
            }
            rslts.push(rslt);
        }
        rslts
    }
    
    /// 矩形フィルタを適用する。
    pub fn sma(input: &[f64], period:usize )->Vec<f64>{
        let period=period as  isize;
        let size = input.len()  as isize;
        let side = (period / 2) as isize;
        let mut rslt = vec![0f64;input.len()];
         
        for i in 0..input.len(){
            let mut total = 0f64;
            if i == 0{
                for j in 0..period{
                   let n = i as isize + side - j as isize;
                   if n >= 0  && n< size{
                      total += input[n as usize];
                   }
                }
            }else{
                total = rslt[i-1];
                let prev = i as isize - 1 - side;
                let cur = i as isize + side;   
                if  0 <= prev && prev < size{
                    total-= input[prev as usize];
                }
                if 0 <= cur && cur < size{
                    total+=input[cur as usize ];
                }
            }
            rslt[i] = total;
         }
         for item in &mut rslt{
            *item /=period as f64;
         }
         
         rslt
    }

    /// 最大値を返す。
    pub fn argmax(input: &[f64]) ->Option<usize>{
        let mut max_val=f64::NEG_INFINITY;
        let mut max_i = 0;
        for i in 0..input.len(){
            if input[i]>max_val{
                max_val=input[i];
                max_i=i;
            }
        } 
        if max_val>f64::NEG_INFINITY{
            Some(max_i)
        }else{
            None
        }

    }

    /// グリッドの生成。
    pub fn grid(input: &[f64], m: usize, margin: f64) -> Vec<f64> {
        let dy = 6.0f64 * Self::mad(&input) / m as f64;

        let mut y_min = input.iter().fold(f64::INFINITY, |acc, x| f64::min(acc, *x));
        let mut y_max = input
            .iter()
            .fold(f64::NEG_INFINITY, |acc, x| f64::max(acc, *x));
        y_min -= margin * dy + 0.0001;
        y_max += (margin+1.0) * dy;
        let mut ybin = Vec::<f64>::new();
        let mut yn = y_min;
        while yn < y_max {
            ybin.push(yn);
            yn += dy;
        }
        ybin
    }

    /// 中央絶対偏差(Median Absolute Deviation)を返す。
    pub fn mad(input: &[f64]) -> f64 {
        match Self::median(input) {
            Some(med) => {
                let sz = input.len();
                let mut v = vec![0f64; sz];
                for i in 0..sz {
                    v[i] = (input[i] - med).abs();
                }
                Self::median(&v).unwrap()
            }
            None => 0.0,
        }
    }
    /// 中央値(Median)を返す。
    pub fn median(input: &[f64]) -> Option<f64> {
        let sz = input.len();
        if sz == 0 {
            return None;
        }
        let mut v = input.to_vec();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if sz == 1 {
            Some(v[0])
        } else {
            let med = sz / 2 ;
            if sz & 1 != 0 {
                Some(v[med])
            } else {
                Some((v[med - 1] + v[med]) * 0.5)
            }
        }
    }
}

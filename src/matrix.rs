#[derive(Debug,Clone)]

/// Matrix それ自身。
pub struct Matrix {
    col_hight: usize,
    row_width: usize,
    table: Vec<Vec<f64>>,
}

impl Matrix {
    /// Matrix の新しいインスタンス作成。
    pub fn new(col_hight: usize, row_width: usize) -> Self {
        let table = vec![vec![0.0f64; row_width];col_hight];

        Self {
            col_hight,
            row_width,
            table,
        }
    }
    /// 単一のアイテムを返します。
    pub fn at(& self,col:usize,row:usize)->f64{
        self.table[col][row]
    }
    
    /// 単一のアイテムを更新します。
    pub fn update(&mut self,col:usize,row:usize,val:f64){
        self.table[col][row]=val;
    }
    
    /// 指定された行のアイテムを返します。
    pub fn get_row(& self,col:usize)->Option<Vec<f64>>{
        if col < self.col_hight{
            Some(self.table[col].to_vec())
        }else{
            None
        }
    }
    /// 指定された行を更新します。
    pub fn set_row(&mut self,col:usize,vals:&[f64])->bool{
        if col < self.col_hight && vals.len()==self.row_width{
            self.table[col]=vals.to_vec();
            true
        }else{
            false
        }
    
    }
    /// 指定された列のアイテムを返します。
    pub fn get_col(& self,row:usize)->Option<Vec<f64>>{
        if row < self.row_width{
            let mut rslt=Vec::<f64>::new();
            for i in 0..self.col_hight{
                rslt.push(self.table[i][row]);
            }
            Some(rslt) 
        }else{
            dbg!(&self.row_width,&row);
            None
        }
    }
    /// 指定された列を更新します。
    pub fn set_col(&mut self,row:usize,vals:&[f64])->bool{
        if row < self.row_width && vals.len()==self.col_hight{
            for i in 0..self.col_hight{
                self.table[i][row]=vals[i];
            }
            true
        }else{
            false
        }
    }
}

use fmgf::*;

fn main() {
    let data = vec![
        5.6, 5.7, 5.4, 5.5, 5.8, 5.5, 5.3, 5.6, 5.4, 8.2, 10.2, 15.0, 3.0, 20.1, 10.2, 6.5, 5.8,
    ];
    dbg!(&data);

    let z = Fmgf::fmgf(&data, 8.0,21);
    dbg!(z);

}

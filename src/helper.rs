trait Round {
    fn to_2_deci(self) -> f64;
}
impl Round for f64 {
    fn to_2_deci(self) -> f64 {
        (self * 100.0).round() / 100.0
    }
}
pub struct Mapper {}
impl Mapper {
    pub fn convert_bytes(size: f64) -> (f64, String) {
        let base: f64 = 1024.00;
        let _kb = base.powf(2.0);
        let _mb = base.powf(3.0);
        let _gb = base.powf(4.0);
        let _tb = base.powf(5.0);
        let _eb = base.powf(6.0);

        let abs: f64;
        if size > base && size <= _kb {
            abs = (size / base).to_2_deci();
            return (abs, "KiB".to_string());
        } else if size > _kb && size <= _mb {
            abs = (size / _kb).to_2_deci();
            return (abs, "MiB".to_string());
        } else if size > _mb && size <= _gb {
            abs = (size / _mb).to_2_deci();
            return (abs, "GiB".to_string());
        } else if size > _gb && size <= _tb {
            abs = (size / _gb).to_2_deci();
            return (abs, "TiB".to_string());
        } else {
            abs = size.to_2_deci();
            return (abs, "Bytes".to_string());
        }
    }
}

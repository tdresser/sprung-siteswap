use crate::lib::data::Pattern;

mod lib;

fn main() {
    /*let pattern = parse("czizTtzS312");
    println!("{:?}\n", pattern);
    println!("{}", pattern.get_canonical_form());*/

    let mut pattern = Pattern::new("czizczizS312");
    println!("{:?}\n", pattern);
    println!("{}", pattern.get_canonical_form());
}

#[cfg(test)]
mod tests {
    use crate::lib::{data::Pattern, data::Position};

    #[test]
    fn box_base() {
        let mut pattern = Pattern::new("cB");
        assert_eq!(*pattern.siteswap(), vec![2]);
        assert_eq!(
            *pattern.arc_positions(),
            vec![(Position::BottomOpposite, Position::BottomOpposite)]
        );
        assert_eq!(pattern.get_canonical_form(), "Sc2");
    }

    #[test]
    fn cascade_base() {
        let mut pattern = Pattern::new("iSC");
        assert_eq!(*pattern.siteswap(), vec![3]);
        assert_eq!(
            *pattern.zip_positions(),
            vec![(Position::TopNatural, Position::TopNatural)]
        );
        assert_eq!(pattern.get_canonical_form(), "izS3");
    }

    #[test]
    fn fountain_base() {
        let mut pattern = Pattern::new("ciSF");
        assert_eq!(*pattern.siteswap(), vec![4]);
        assert_eq!(
            *pattern.arc_positions(),
            vec![(Position::TopOpposite, Position::TopOpposite)]
        );
        assert_eq!(pattern.get_canonical_form(), "izScF");
    }

    #[test]
    fn sprung_base() {
        let pattern = Pattern::new("iczS312");
        assert_eq!(*pattern.siteswap(), vec![3, 1, 2]);
        assert_eq!(
            *pattern.zip_positions(),
            vec![(Position::TopOpposite, Position::TopOpposite)]
        );
    }

    #[test]
    fn collapse_zipped() {
        let mut pattern = Pattern::new("izicziziczB");
        assert_eq!(pattern.get_canonical_form(), "izcizS2");
    }

    #[test]
    fn collapse_siteswap_positions() {
        let mut pattern = Pattern::new("Sc3c1n2");
        assert_eq!(pattern.get_canonical_form(), "Sc31n2");
    }

    #[test]
    fn invalid_siteswap() {
        let pattern = Pattern::new("S321");
        assert_eq!(*pattern.error(), Some("Invalid siteswap.".to_string()));
    }
}

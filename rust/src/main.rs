use crate::lib::data::Pattern;

mod lib;

fn main() {
    /*let pattern = parse("czizTtzS312");
    println!("{:?}\n", pattern);
    println!("{}", pattern.get_canonical_form());*/

    let mut pattern = Pattern::new("czizczizS312");
    println!("{:?}\n", pattern);
    println!("{}", pattern.get_canonical_form());

    /*pattern = parse("cB");
    println!("{:?}\n", pattern);

    pattern = parse("izcB");
    println!("{:?}\n", pattern);

    pattern = parse("czSc3ci1n2");
    println!("{:?}\n", pattern);

    pattern = parse("izczSc3ci1n2");
    println!("{:?}\n", pattern);

    pattern = parse("bbzSTb3c1ic2");
    println!("{:?}\n", pattern);

    pattern = parse("TbzBbB");
    println!("{:?}\n", pattern);*/
}

#[cfg(test)]
mod tests {
    use crate::lib::{data::Pattern, data::Position};

    #[test]
    fn box_base() {
        let pattern = Pattern::new("cB");
        assert_eq!(*pattern.siteswap(), vec![2]);
        assert_eq!(
            *pattern.nonzip_positions(),
            vec![(Position::BottomOpposite, Position::BottomOpposite)]
        );
    }

    #[test]
    fn cascade_base() {
        let pattern = Pattern::new("iC");
        assert_eq!(*pattern.siteswap(), vec![3]);
        assert_eq!(
            *pattern.nonzip_positions(),
            vec![(Position::TopNatural, Position::TopNatural)]
        );
    }

    #[test]
    fn fountain_base() {
        let pattern = Pattern::new("ciF");
        assert_eq!(*pattern.siteswap(), vec![4]);
        assert_eq!(
            *pattern.nonzip_positions(),
            vec![(Position::TopOpposite, Position::TopOpposite)]
        );
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

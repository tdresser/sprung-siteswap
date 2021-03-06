use crate::lib::data::Pattern;

mod lib;

fn main() {
    /*let pattern = parse("czizTtzS312");
    println!("{:?}\n", pattern);
    println!("{}", pattern.get_canonical_form());*/

    let mut pattern = Pattern::new("izS42");
    println!("{:?}\n", pattern);
    println!("{}", pattern.get_canonical_form());
    println!("{}", pattern.get_hand_positions());
}

#[cfg(test)]
mod tests {
    use crate::lib::{data::Pattern, data::Position};

    #[test]
    fn bare() {
        assert_eq!(Pattern::new("B").get_canonical_form(), "S2");
        assert_eq!(Pattern::new("SC").get_canonical_form(), "S3");
        assert_eq!(Pattern::new("SF").get_canonical_form(), "S4");
        assert_eq!(Pattern::new("S5").get_canonical_form(), "S5");
    }

    #[test]
    fn cib() {
        assert_eq!(Pattern::new("ciB").get_canonical_form(), "izSc2");
    }

    #[test]
    fn parse_failure() {
        let pattern = Pattern::new("C");
        // TODO, maybe try to clean this up?
        assert_eq!(
            *pattern.error(),
            Some(" --> 1:1\n  |\n1 | C\n  | ^---\n  |\n  = expected notation".to_string())
        );
    }

    #[test]
    fn infix_arcs() {
        assert!(Pattern::new("Bc").error().is_some());
        assert_eq!(Pattern::new("ScC").get_canonical_form(), "Sc3");
        assert_eq!(Pattern::new("ScF").get_canonical_form(), "Sc4");
        assert_eq!(Pattern::new("Sc5i1").get_canonical_form(), "Sc5i1");
        assert_eq!(Pattern::new("Sc5n1").get_canonical_form(), "Sc5n1");
    }

    #[test]
    fn infix_arcs_missing_first() {
        assert_eq!(Pattern::new("S5i1").get_canonical_form(), "S5i1");
    }

    #[test]
    fn explicit_arcs() {
        assert_eq!(Pattern::new("cacianaB").get_canonical_form(), "Sc2ci2n2");
    }

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
        let mut pattern = Pattern::new("izcaSF");
        assert_eq!(*pattern.siteswap(), vec![4]);
        assert_eq!(
            *pattern.zip_positions(),
            vec![(Position::TopNatural, Position::TopNatural)]
        );
        assert_eq!(
            *pattern.arc_positions(),
            vec![(Position::BottomOpposite, Position::BottomOpposite)]
        );
        assert_eq!(pattern.get_canonical_form(), "izSc4");
    }

    #[test]
    fn explicit_arcs_ambiguous_zip() {
        assert_eq!(Pattern::new("caciaiB").get_canonical_form(), "izSc2ci2");
    }

    #[test]
    fn explicit_zips_ambiguous_arc() {
        assert_eq!(Pattern::new("czciznzcB").get_canonical_form(), "czciznzSc2");
    }

    #[test]
    fn explicit_zips_ambiguous_zip() {
        assert!(Pattern::new("czciziB").error().is_some());
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
    fn collapse_zips() {
        let mut pattern = Pattern::new("izicziziczB");
        assert_eq!(pattern.get_canonical_form(), "izcizS2");
    }

    #[test]
    fn collapse_arcs() {
        let mut pattern = Pattern::new("iaicaiaicaB");
        assert_eq!(pattern.get_canonical_form(), "Si2ci2");
    }

    #[test]
    fn collapse_siteswap() {
        let mut pattern = Pattern::new("S531531");
        assert_eq!(pattern.get_canonical_form(), "S531");
    }

    #[test]
    fn rotate_siteswap() {
        let mut pattern = Pattern::new("S315");
        assert_eq!(pattern.get_canonical_form(), "S531");
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

    #[test]
    fn traditional_siteswap() {
        assert_eq!(
            Pattern::new("S423").get_traditional_siteswap(),
            "(8,2x)(2x,4)(6x,2x)*"
        );
    }

    #[test]
    fn hex_siteswap() {
        assert_eq!(
            Pattern::new("S51").get_traditional_siteswap(),
            "(ax,2x)(2x,2x)"
        );
    }

    #[test]
    fn hand_positions() {
        assert_eq!(
            Pattern::new("iB").get_hand_positions(),
            "(20)(30, 50).(10, 50)(20).(10, 50)(20).(20)(30, 50)."
        );
    }

    #[test]
    fn hand_position_sequence() {
        let pattern = Pattern::new("izczB");
        assert_eq!(
            pattern.get_hand_positions(),
            "(20)(-30).(10, 50)(20).(-10)(20).(20)(30, 50)."
        );
        assert_eq!(pattern.get_traditional_siteswap(), "(4,2x)(2x,4)");
    }
}

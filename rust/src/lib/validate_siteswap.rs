pub fn validate_siteswap(ss: &Vec<u32>) -> Result<(), String> {
    let mut catch_count = vec![0; ss.len()];

    for i in 0..ss.len() {
        let catch_index = (i + ss[i] as usize) % ss.len();
        catch_count[catch_index] += 1;
        if catch_count[catch_index] > 1 {
            return Err("Invalid siteswap.".to_string());
        }
    }

    return Ok(());
}

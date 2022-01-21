use notation_tab::prelude::*;

pub mod test;
pub mod songs;
pub mod classical;
pub mod beginner;

pub fn main() {
    write_tab(&test::new_tab(), "tabs/test.ron");
    //songs
    write_tab(&songs::scarborough_fair::new_tab(), "tabs/songs/scarborough_fair.ron");
    //classical
    write_tab(&classical::canon_in_d::new_tab(), "tabs/classical/canon_in_d.ron");
    //beginner
    write_tab(&beginner::right_hand_1::new_tab(), "tabs/beginner/right_hand_1.ron");
    write_tab(&beginner::right_hand_1::new_tab(), "tabs/beginner/right_hand_1.ron");
    write_tab(&beginner::g_major_3rd_string::new_tab(), "tabs/beginner/g_major_3rd_string.ron");
}

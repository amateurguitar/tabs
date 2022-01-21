#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! notation_tab = "0.4.0"
//! ``` 

use notation_tab::prelude::*;

pub fn main() {
    print_tab(&new_tab());
}

pub fn new_tab() -> Tab {
    tab! {
        "481564a9-1f37-45d3-83f4-492f643fb8be"
        Meta: D Major 4 _4 80
        Tracks: [
            {chord Chord [
                $duration = _1_2
                "i:1"
                    Chord ( 1: 3 5 )
                    Chord ( 5: 3 5 ) |
                "i:2"
                    Chord ( 6: 3- 5 )
                    Chord ( 3: 3- 5 ) |
                "i:3"
                    Chord ( 4: 3 5 )
                    Chord ( 1: 3 5 ) |
                "i:4"
                    Chord ( 2: 3- 5 7- /3-)
                    Chord ( 5: 3 5 ) |
                $duration = _1
                "ending" Chord (1: 3 5) |
            ]}
            {guitar Guitar [
                Fretboard tuning: DropD
                $duration = _1_2
                "i:1"
                    "D" Shape ( 0 0 4 2 3 2 ) 
                    "A" Shape ( _ 0 2 2 2 0 ) |
                "i:2"
                    "Bm" Shape ( _ 2 4 4 3 2 ) 
                    "F#m" Shape ( 4 4 4 2 2 2 ) |
                "i:3"
                    "G" Shape ( 5 2 0 0 0 _ ) 
                    "D" Shape ( 0 5 4 2 3 2 ) |
                "i:4"
                    "Em7/B" Shape ( 5 2 0 0 0 0 ) 
                    "A" Shape ( _ 0 2 2 2 0 ) |
                $duration = _1_8
                "pi:1" Pick [ 6 4 3 2 5 4 3 2 ] |
                "pi:2" Pick [ 5 4@0 4@4 2@0 6 5 4 3 ] |
                "pi:3" Pick [ 6 5 4 3 6 5 4 3 ] |
                "pi:4" Pick [ 6 4 3 2 5 4 3 2 ] |
                "pa:1" Pick [ (6 1) 4 3 2 (5 1) 4 3 2 ] |
                "pa:2" Pick [ (5 2) 4@0 4@4 3@4 (6 2) 5 4 3 ] |
                "pa:3" Pick [ (6 2) 5 4 3 (6 3) 5@0 4@0 4@4 ] |
                "pa:4" Pick [ (6 2) 5 4 3 (5@0 2) 5@4 4 3 ] |
                "pa:5" Pick [ (6 2 1) 4 3 2 (5 2 1) 4 3 2 ] |
                "pa:6" Pick [ (5 3 2) 4@0 4@4 3@4 (6 3 2) 5 4 3 ] |
                "pa:7" Pick [ (6 3 2) 5 4 3 (6 4 3) 5@0 4@0 4@4 ] |
                "pa:8" Pick [ (6 3 2) 5 4 3 (5@0 3 2) 5@4 4 3 ] |
                $duration = _1_4
                "pb:1" Pick [ (6 3 2 1) 4 (5 3 2 1) 3 ] |
                "pb:2" Pick [ (5 4 3 2) 4 (6 4 3 2) 4 ] |
                "pb:3" Pick [ (6 4 3 2) 5 (6 4 3) 3 ] |
                "pb:4" Pick [ (6 3 2) 2 (5 3 2) 3@0 ] |
                "pb:5" Pick [ (6 4 2) 4 (5 3 2) 3 ] |
                "pb:6" Pick [ (5 4 2) 1 (6 1) 3 ] |
                "pb:7" Pick [ (6 3 2) 3 (5 4 3) (4 3) ] |
                "pb:8" Pick [ (6 4 3) (3 2) (5 2) 3@0, 2, ] |
                $duration = _1_8
                "pc:1" Pick [ (6 4@0 2) 4@2 4@4 3@0 (5 3) 4 3 2@0 ] |
                "pc:2" Pick [ (5 4) 2@0 3@2 3@0 (5 3@2) 3@0 4@4 4@2 ] |
                "pc:3" Pick [ (6 4) 5 2@0 2@2 (6 2@3) 2@2 2@0 3 ] |
                "pc:4" Pick [ (6 3) 4@4 4@2 2 (5 3) 2@0 3@2 3@0 ] |
                "pc:5" Pick [ (6 2) 4 (4 1) 3@0 (5 3 1) 4 3 3@0 ] |
                "pc:6" Pick [ (5 4) 2@0 (3 2) 3@0 (6 3 1) 3@0 4 4@2 ] |
                "pc:7" Pick [ (6 4 2) 5 5 5@4 (6 4 3) 5@4 (5@2 2@0) 5@0 ] |
                "pc:8" Pick [ (6 3 2) 4@4 (4@2 1@3) 2 (5 3 1) 3@0 (4@4 1@5) 4 ] |
                $duration = _1_16
                "pd:1" Pick [ (6 1@5 *) 1@2 1@3 1@5* 1@2 1@3 (5 1@5) 3@2 2@0 2@2 2@3 1@0 1@2 1@3 ] |
                "pd:2" Pick [ (5 1@2 *) 2@3 1@0 1@2* 4 3@0 (6 3@2) 2@2 3@2 3@0 3@2 4 3@0 3@2 ] |
                "pd:3" Pick [ (6 3@0 *) 2@0 3@2 3@0* 4@4 4@2 (6 4@4) 4@2 4@0 4@2 4@4 3@0 3@2 2@0 ] |
                "pd:4" Pick [ (6 3@0 *) 2@0 3@2 2@0* 2@2 2@3 (5 3@2) 2@0 2@2 2@3 1@0 1@2 1@3 1@5 ] |
                $duration = _1_8
                "pe:1" Pick [ (6 2 1) 2, 1@0, 1 1 (5 2) 2, 2@3, 1 2 ] |
                "pe:2" Pick [ (5 2@0) 2@3, 1@0, 1@2 2@3 (6 1) 1, 1@0, 2@3 2 ] |
                "pe:3" Pick [ (6 2) 2, 3@2, 2 2@2 (6 2@3) 1@2, 1@0, 1@2 1@5 ] |
                "pe:4" Pick [ (6 1@3) 2@3, 3@6, 2 2 (5 3) 3, 3@0, 4@4 4 ] |
                "pe:5" Pick [ (6 1) 4 3@0 4 (5 4) 1 1@2 1 ] |
                "pe:6" Pick [ (5 2) 4 4@0 2@0 (6 3) 5@0 6@5 5@0 ] |
                "pe:7" Pick [ (6 5) 2 2@2 2 (6 3) 5 6@5 5] |
                "pe:8" Pick [ (6 5) 2 3@2 2 (5 2) 5@4 5@2 5@4 ] |
                "pe:9" Pick [ (6 4) 2 1@0 2 (5 2) 5@4 4@0 5@4 ] |
                "pe:10" Pick [ 5 2@0 3@2 2@0 (6 2) 5 4 4@2 ] |
                "pe:11" Pick [ (6 4) 2@3 1@0 1@3 (6 1@2) 4@4 3@2 1@2 ] |
                "pe:12" Pick [ (6 2@3) 1@3 1@2 1@3 (5 1@0) 1@5 1@3 1@5 ] |
                "pf:1" Pick [ (6 2@7 1@5 *+) (2@7 1@5) (5 2@7 1@5) (2@8 1@7) (2@7 1@5) (2@5 1@3) ] |
                "pf:2" Pick [ (5 2 1 *+) 1 (4 1@2) 1@3 1@2 1@0 ] |
                "pf:3" Pick [ (6 2@3) 2@1 2@0 2@1 (6 4@4 3 *+) 3 ] |
                "pf:4" Pick [ (6 4@2 3 *) 2@3* (5 3 2 *) 3@0 2 ] |
                $duration = _1_4
                "pf:5" Pick [ (6 4 3 2) 4 5 4@2 ] |
                "pf:6" Pick [ (5 4@0) (3 2) 6 (3@2 2@1) ] |
                "pf:7" Pick [ (6 3 2 *) (6 2@3) 3 ] |
                "pf:8" Pick [ (6 3) 1 (5 2 +) 2, ] |
                "pf:9" Pick [ (6 2 1) (4 2) (5 4) (2 1) ] |
                "pf:10" Pick [ (5 4 3 2) 4@0 (5 4) (3 2) ] |
                "pf:11" Pick [ (6 3 2) (2@8 1@7) (6 2@7 1@5) (4@4 3) ] |
                "pf:12" Pick [ (6 4@2 3) 1 (5 4 3) (2 1) ] |
                $duration = _1_4
                "ending" "D" Shape ( 0 0 0 2 3 2 ) | 
                "ending:p" Pick [ (6 5 4 3 2 1) ] | 
            ]}
        ]
        Sections: [
            {intro Intro [
                {
                    chord [ "i:1" | ]
                    guitar [ "i:1" | ; "pi:1" | ]
                } {
                    chord [ "i:2" | ]
                    guitar [ "i:2" | ; "pi:2" | ]
                } {
                    chord [ "i:3" | ]
                    guitar [ "i:3" | ; "pi:3" | ]
                } {
                    chord [ "i:4" | ]
                    guitar [ "i:4" | ; "pi:4" | ]
                }
            ]}
            {a Verse [
                {
                    chord [ "i:1" | ]
                    guitar [ "i:1" | ; "pa:1" | ]
                } {
                    chord [ "i:2" | ]
                    guitar [ "i:2" | ; "pa:2" | ]
                } {
                    chord [ "i:3" | ]
                    guitar [ "i:3" | ; "pa:3" | ]
                } {
                    chord [ "i:4" | ]
                    guitar [ "i:4" | ; "pa:4" | ]
                }
                {
                    chord [ "i:1" | ]
                    guitar [ "i:1" | ; "pa:5" | ]
                } {
                    chord [ "i:2" | ]
                    guitar [ "i:2" | ; "pa:6" | ]
                } {
                    chord [ "i:3" | ]
                    guitar [ "i:3" | ; "pa:7" | ]
                } {
                    chord [ "i:4" | ]
                    guitar [ "i:4" | ; "pa:8" | ]
                }
            ]}
            {b Verse [
                {
                    chord [ "i:1" | ]
                    guitar [ "i:1" | ; "pb:1" | ]
                } {
                    chord [ "i:2" | ]
                    guitar [ "i:2" | ; "pb:2" | ]
                } {
                    chord [ "i:3" | ]
                    guitar [ "i:3" | ; "pb:3" | ]
                } {
                    chord [ "i:4" | ]
                    guitar [ "i:4" | ; "pb:4" | ]
                }
                {
                    chord [ "i:1" | ]
                    guitar [ "i:1" | ; "pb:5" | ]
                } {
                    chord [ "i:2" | ]
                    guitar [ "i:2" | ; "pb:6" | ]
                } {
                    chord [ "i:3" | ]
                    guitar [ "i:3" | ; "pb:7" | ]
                } {
                    chord [ "i:4" | ]
                    guitar [ "i:4" | ; "pb:8" | ]
                }
            ]}
            {c Verse [
                {
                    chord [ "i:1" | ]
                    guitar [ "i:1" | ; "pc:1" | ]
                } {
                    chord [ "i:2" | ]
                    guitar [ "i:2" | ; "pc:2" | ]
                } {
                    chord [ "i:3" | ]
                    guitar [ "i:3" | ; "pc:3" | ]
                } {
                    chord [ "i:4" | ]
                    guitar [ "i:4" | ; "pc:4" | ]
                }
                {
                    chord [ "i:1" | ]
                    guitar [ "i:1" | ; "pc:5" | ]
                } {
                    chord [ "i:2" | ]
                    guitar [ "i:2" | ; "pc:6" | ]
                } {
                    chord [ "i:3" | ]
                    guitar [ "i:3" | ; "pc:7" | ]
                } {
                    chord [ "i:4" | ]
                    guitar [ "i:4" | ; "pc:8" | ]
                }
            ]}
            {d Bridge [
                {
                    chord [ "i:1" | ]
                    guitar [ "i:1" | ; "pd:1" | ]
                } {
                    chord [ "i:2" | ]
                    guitar [ "i:2" | ; "pd:2" | ]
                } {
                    chord [ "i:3" | ]
                    guitar [ "i:3" | ; "pd:3" | ]
                } {
                    chord [ "i:4" | ]
                    guitar [ "i:4" | ; "pd:4" | ]
                }
            ]}
            {e Chorus [
                {
                    chord [ "i:1" | ]
                    guitar [ "i:1" | ; "pe:1" | ]
                } {
                    chord [ "i:2" | ]
                    guitar [ "i:2" | ; "pe:2" | ]
                } {
                    chord [ "i:3" | ]
                    guitar [ "i:3" | ; "pe:3" | ]
                } {
                    chord [ "i:4" | ]
                    guitar [ "i:4" | ; "pe:4" | ]
                }
                {
                    chord [ "i:1" | ]
                    guitar [ "i:1" | ; "pe:5" | ]
                } {
                    chord [ "i:2" | ]
                    guitar [ "i:2" | ; "pe:6" | ]
                } {
                    chord [ "i:3" | ]
                    guitar [ "i:3" | ; "pe:7" | ]
                } {
                    chord [ "i:4" | ]
                    guitar [ "i:4" | ; "pe:8" | ]
                }
                {
                    chord [ "i:1" | ]
                    guitar [ "i:1" | ; "pe:9" | ]
                } {
                    chord [ "i:2" | ]
                    guitar [ "i:2" | ; "pe:10" | ]
                } {
                    chord [ "i:3" | ]
                    guitar [ "i:3" | ; "pe:11" | ]
                } {
                    chord [ "i:4" | ]
                    guitar [ "i:4" | ; "pe:12" | ]
                }
            ]}
            {f Chorus [
                {
                    chord [ "i:1" | ]
                    guitar [ "i:1" | ; "pf:1" | ]
                } {
                    chord [ "i:2" | ]
                    guitar [ "i:2" | ; "pf:2" | ]
                } {
                    chord [ "i:3" | ]
                    guitar [ "i:3" | ; "pf:3" | ]
                } {
                    chord [ "i:4" | ]
                    guitar [ "i:4" | ; "pf:4" | ]
                }
                {
                    chord [ "i:1" | ]
                    guitar [ "i:1" | ; "pf:5" | ]
                } {
                    chord [ "i:2" | ]
                    guitar [ "i:2" | ; "pf:6" | ]
                } {
                    chord [ "i:3" | ]
                    guitar [ "i:3" | ; "pf:7" | ]
                } {
                    chord [ "i:4" | ]
                    guitar [ "i:4" | ; "pf:8" | ]
                }
                {
                    chord [ "i:1" | ]
                    guitar [ "i:1" | ; "pf:9" | ]
                } {
                    chord [ "i:2" | ]
                    guitar [ "i:2" | ; "pf:10" | ]
                } {
                    chord [ "i:3" | ]
                    guitar [ "i:3" | ; "pf:11" | ]
                } {
                    chord [ "i:4" | ]
                    guitar [ "i:4" | ; "pf:12" | ]
                }
            ]}
            {ending Outro [
                {
                    chord [ "ending" | ]
                    guitar [ "ending" | ; "ending:p" | ]
                }
            ]}
        ]
        Form: intro a b c d d e f ending
    }
}


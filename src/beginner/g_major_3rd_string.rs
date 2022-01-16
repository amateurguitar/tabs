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
        "10066c3f-9307-43fc-a9b4-95cf80242330"
        Meta: G Major 4 _4 60
        Tracks: [
            {chord Chord [
                $duration = _1
                "1" Chord ( 1: 1 )
            ]}
            {guitar Guitar [
                Fretboard
                $duration = _1
                "1" Shape ( _ _ _ 0 _ _ )
                $duration = _1_8
                "a:1" Pick [ 3@0 3@2 3@4 3@5 3@7 3@9 3@11 3@12 ] |
                "a:2" Pick [ 3@12 3@11 3@9 3@7 3@5 3@4 3@2 3@0 ] |
                "b:1" Pick [ 3@0 3@0 3@2 3@2 3@4 3@4 3@5 3@5 ] |
                "b:2" Pick [ 3@7 3@7 3@9 3@9 3@11 3@11 3@12 3@12 ] |
                "b:3" Pick [ 3@12 3@12 3@11 3@11 3@9 3@9 3@7 3@7 ] |
                "b:4" Pick [ 3@5 3@5 3@4 3@4 3@2 3@2 3@0 3@0 ] |
                "c:1" Pick [ 3@0 3@4 3@2 3@5 3@4 3@7 3@5 3@9 ] |
                "c:2" Pick [ 3@7 3@11 3@9 3@12 3@12 3@9 3@11 3@7 ] |
                "c:3" Pick [ 3@9 3@5 3@7 3@4 3@5 3@2 3@4 3@0 ] |
                "d:1" Pick [ 3@0 3@2 3@4 3@0 3@2 3@4 3@5 3@2 ] |
                "d:2" Pick [ 3@4 3@5 3@7 3@4 3@5 3@7 3@9 3@5 ] |
                "d:3" Pick [ 3@7 3@9 3@11 3@7 3@9 3@11 3@12 3@9 ] |
                "d:4" Pick [ 3@12 3@11 3@9 3@12 3@11 3@9 3@7 3@11 ] |
                "d:5" Pick [ 3@9 3@7 3@5 3@9 3@7 3@5 3@4 3@7 ] |
                "d:6" Pick [ 3@5 3@4 3@2 3@5 3@4 3@2 3@0 3@4 ] |
                "e:1" Pick [ 3@0 3@2 3@4* 3@2 3@4 3@5* ] |
                "e:2" Pick [ 3@4 3@5 3@7* 3@5 3@7 3@9* ] |
                "e:3" Pick [ 3@7 3@9 3@11* 3@9 3@11 3@12* ] |
                "e:4" Pick [ 3@12 3@11 3@9* 3@11 3@9 3@7* ] |
                "e:5" Pick [ 3@9 3@7 3@5* 3@7 3@5 3@4* ] |
                "e:6" Pick [ 3@5 3@4 3@2* 3@4 3@2 3@0* ] |
                "b1:1" Pick [ 3@0 3@0 3@0 3@2 3@0 3@4 3@0 3@5 ] |
                "b1:2" Pick [ 3@0 3@7 3@0 3@9 3@0 3@11 3@0 3@12 ] |
                "b1:3" Pick [ 3@0 3@12 3@0 3@11 3@0 3@9 3@0 3@7 ] |
                "b1:4" Pick [ 3@0 3@5 3@0 3@4 3@0 3@2 3@0 3@0 ] |
                "b2:1" Pick [ 3@0 3@0 3@2 3@0 3@4 3@0 3@5 3@0 ] |
                "b2:2" Pick [ 3@7 3@0 3@9 3@0 3@11 3@0 3@12 3@0 ] |
                "b2:3" Pick [ 3@12 3@0 3@11 3@0 3@9 3@0 3@7 3@0 ] |
                "b2:4" Pick [ 3@5 3@0 3@4 3@0 3@2 3@0 3@0 3@0 ] |
                "b3:1" Pick [ 3@0= 3@0- 3@2= 3@2- 3@4= 3@4- 3@5= 3@5- ] |
                "b3:2" Pick [ 3@7= 3@7- 3@9= 3@9- 3@11= 3@11- 3@12= 3@12- ] |
                "b3:3" Pick [ 3@12= 3@12- 3@11= 3@11- 3@9= 3@9- 3@7= 3@7- ] |
                "b3:4" Pick [ 3@5= 3@5- 3@4= 3@4- 3@2= 3@2- 3@0= 3@0- ] |
                "b4:1" Pick [ 3@0= 3@0- 3@0= 3@2- 3@0= 3@4- 3@0= 3@5- ] |
                "b4:2" Pick [ 3@0= 3@7- 3@0= 3@9- 3@0= 3@11- 3@0= 3@12- ] |
                "b4:3" Pick [ 3@0= 3@12- 3@0= 3@11- 3@0= 3@9- 3@0= 3@7- ] |
                "b4:4" Pick [ 3@0= 3@5- 3@0= 3@4- 3@0= 3@2- 3@0= 3@0- ] |
                "b5:1" Pick [ 3@0= 3@0- 3@2= 3@0- 3@4= 3@0- 3@5= 3@0- ] |
                "b5:2" Pick [ 3@7= 3@0- 3@9= 3@0- 3@11= 3@0- 3@12= 3@0- ] |
                "b5:3" Pick [ 3@12= 3@0- 3@11= 3@0- 3@9= 3@0- 3@7= 3@0- ] |
                "b5:4" Pick [ 3@5= 3@0- 3@4= 3@0- 3@2= 3@0- 3@0= 3@0- ] |
            ]}
        ]
        Sections: [
            {a Verse [
                {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "a:1" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "a:2" | ]
                }
            ]}
            {b Verse [
                {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b:1" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b:2" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b:3" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b:4" | ]
                }
            ]}
            {c Verse [
                {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "c:1" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "c:2" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "c:3" | ]
                }
            ]}
            {d Verse [
                {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "d:1" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "d:2" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "d:3" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "d:4" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "d:5" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "d:6" | ]
                }
            ]}
            {e Verse [
                {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "e:1" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "e:2" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "e:3" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "e:4" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "e:5" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "e:6" | ]
                }
            ]}
            {b1 Verse [
                {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b1:1" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b1:2" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b1:3" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b1:4" | ]
                }
            ]}
            {b2 Verse [
                {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b2:1" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b2:2" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b2:3" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b2:4" | ]
                }
            ]}
            {b3 Verse [
                {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b3:1" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b3:2" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b3:3" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b3:4" | ]
                }
            ]}
            {b4 Verse [
                {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b4:1" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b4:2" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b4:3" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b4:4" | ]
                }
            ]}
            {b5 Verse [
                {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b5:1" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b5:2" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b5:3" | ]
                } {
                    chord [ "1" 1 ]
                    guitar [ "1" 1 ; "b5:4" | ]
                }
            ]}
        ]
        Form: a b c d e b1 b2 b3 b4 b5
    }
}

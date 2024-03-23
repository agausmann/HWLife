use <../backplane/backplane.scad>
use <h-link.scad>
use <x-link.scad>

module backplane()
{
    TXFM_1();
}

translate([ 0, 0, 10 ])
{
    backplane();
    translate([ 100, 0, 0 ]) backplane();
    translate([ 0, 100, 0 ]) backplane();
    translate([ 100, 100, 0 ]) backplane();
}

translate([ -25, 25, 0 ]) h_link();
translate([ 75, -25, 0 ]) rotate([ 0, 0, 90 ]) h_link();
translate([ 25, 25, 0 ]) x_link();

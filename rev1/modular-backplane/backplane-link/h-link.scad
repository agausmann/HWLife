include <common.scad>

module h_link_uncut()
{
    for (i = [ 0, 1 ])
        for (j = [ 0, 1 ])
            translate([ i * intra_board_width, j * inter_board_width ]) peg();

    for (i = [ 0, 1 ])
        translate([ i * intra_board_width, 0, 0 ]) rotate([ 0, 0, 90 ]) link(inter_board_width);

    translate([ 0, 0.5 * inter_board_width, 0 ]) link(intra_board_width);
}

module h_link()
{
    intersection()
    {
        cube([ intra_board_width, inter_board_width, total_height ]);
        h_link_uncut();
    }
}

h_link_uncut();

include <common.scad>

module x_link_uncut()
{
    for (i = [ 0, 1 ])
        for (j = [ 0, 1 ])
            translate([ i * intra_board_width, j * inter_board_width ]) peg();

    rotate([ 0, 0, atan(inter_board_width / intra_board_width) ])
        link(sqrt(pow(intra_board_width, 2) + pow(inter_board_width, 2)));

    translate([ 0, inter_board_width, 0 ]) rotate([ 0, 0, -atan(inter_board_width / intra_board_width) ])
        link(sqrt(pow(intra_board_width, 2) + pow(inter_board_width, 2)));
}

module x_link()
{
    intersection()
    {
        cube([ intra_board_width, inter_board_width, total_height ]);
        x_link_uncut();
    }
}

x_link();

$fs = 0.1;

mm = 1;

intra_board_width = 50 * mm;
inter_board_width = 50 * mm;

link_width = 4 * mm;
link_height = 2 * mm;

standoff_width = 4 * mm;
standoff_height = 10 * mm;

peg_width = 2 * mm;
peg_height = 2 * mm;

total_height = standoff_height + peg_height;

module peg()
{
    cylinder(h = standoff_height, d = standoff_width);
    translate([ 0, 0, standoff_height ]) cylinder(h = peg_height, d = peg_width);
}

module link(length)
{
    translate([ 0, -0.5 * link_width, 0 ]) cube([ length, link_width, link_height ]);
}

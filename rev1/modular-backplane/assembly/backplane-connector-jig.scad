mm = 1;
eps = 0.01 * mm;

connector_width = 10.15 * mm;
connector_depth = 3.45 * mm;
jig_height = 1 * mm;

cutout_width = 8 * mm;
tab_depth = 2 * mm;

outer_slack = 0.2 * mm;

nrows = 8;
pitch = 12.5 * mm;
wall_width = 1 * mm;
wall_height = 2 * mm;
standoff_height = 1 * mm;
total_width = 2 * (wall_width + outer_slack) + nrows * pitch;

difference()
{
    translate([ -wall_width - outer_slack, -wall_width - outer_slack, 0 ])
        cube([ total_width, total_width, jig_height + wall_height ]);
    translate([ -outer_slack, -outer_slack, jig_height ])
        cube([ nrows * pitch + 2 * outer_slack, nrows * pitch + 2 * outer_slack, wall_height + eps ]);
    for (i = [0:nrows - 1])
    {
        for (j = [0:nrows - 1])
        {
            translate([ i * pitch + (pitch - cutout_width) / 2, j * pitch + wall_width / 2, -eps ])
                cube([ cutout_width, pitch - wall_width, jig_height + 2 * eps ]);
            translate([ i * pitch + (pitch - connector_width) / 2, j * pitch + (pitch - connector_depth) / 2, -eps ])
                cube([ connector_width, connector_depth, jig_height + 2 * eps ]);
        }
    }
}

for (j = [0:nrows])
{
    translate([ -wall_width, j * pitch - wall_width / 2, 0 ])
        cube([ total_width, wall_width, jig_height + standoff_height ]);
}

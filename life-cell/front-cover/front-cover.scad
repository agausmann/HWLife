$fs = 0.1;

mm = 1;
inch = 25.4 * mm;

WIDTH = 1.75 * inch;
HEIGHT = 1.75 * inch;
DEPTH = 0.12 * inch;
EDGE_RADIUS = 1.27 * mm;

HOLE_INSET = 0.175 * inch;
HOLE_RADIUS = 3.2 * mm / 2;

BUTTON_POS = [ 0.495 * inch, 1.225 * inch ];
BUTTON_RADIUS = 1 * mm;

module front_cover()
{
    color([ 1, 1, 1 ], alpha = 0.5) linear_extrude(height = DEPTH) difference()
    {
        hull()
        {
            translate([ EDGE_RADIUS, EDGE_RADIUS ]) circle(r = EDGE_RADIUS);
            translate([ WIDTH - EDGE_RADIUS, EDGE_RADIUS ]) circle(r = EDGE_RADIUS);
            translate([ EDGE_RADIUS, HEIGHT - EDGE_RADIUS ]) circle(r = EDGE_RADIUS);
            translate([ WIDTH - EDGE_RADIUS, HEIGHT - EDGE_RADIUS ]) circle(r = EDGE_RADIUS);
        }

        translate([ HOLE_INSET, HOLE_INSET ]) circle(r = HOLE_RADIUS);
        translate([ WIDTH - HOLE_INSET, HOLE_INSET ]) circle(r = HOLE_RADIUS);
        translate([ HOLE_INSET, HEIGHT - HOLE_INSET ]) circle(r = HOLE_RADIUS);
        translate([ WIDTH - HOLE_INSET, HEIGHT - HOLE_INSET ]) circle(r = HOLE_RADIUS);

        translate(BUTTON_POS) circle(r = BUTTON_RADIUS);
    }
}

front_cover();
// Toddler-Proof Doorknob Cover - v11
// Two-piece split shell with M3 heat-insert screw bosses
//
// Designed to fit over a standard round doorknob (~51mm dia).
// Split along the Y=0 plane into Half A (heat insert side) and Half B (screw side).
// Joined with 5x M3 screws + heat-set inserts:
//   - 4 side bosses (2 at dome equator, 2 at mid-collar) on left/right of X axis
//   - 1 top boss at the dome apex
//
// Print Half A and Half B as separate files (see designs/doorknob_guard/).
// Heat-press M3 inserts into Half A bosses, then screw Half B onto Half A.

knob_r     = 25.5;
stem_r     = 14.5;
sphere_cz  = 19.5;

wall       = 3.0;
gap        = 1.5;

inner_r    = knob_r + gap;     // 27.0
outer_r    = inner_r + wall;   // 30.0

// Collar: keep wall thickness = 3mm all the way down
collar_ir  = stem_r + 0.8;         // 15.3
collar_or  = collar_ir + wall;     // 18.3
collar_len = 12.0;

// M3 heat insert parameters
boss_r       = 4.5;   // boss pad radius
boss_h       = 7.0;   // boss height (protrusion)
insert_r     = 2.35;  // M3 heat insert hole radius (4.7 mm dia)
insert_depth = 6.0;   // heat insert pocket depth
clearance_r  = 1.7;   // M3 screw clearance hole radius (3.4 mm dia)

// Side boss positions: OUTSIDE the shell wall
boss_xz = [
    [ 31.5,  0 ],
    [-31.5,  0 ],
    [ 26.2, -6 ],
    [-26.2, -6 ],
];

// Top boss: each half gets its own full cylinder at the dome apex,
// protruding toward the other half (along Y), meeting at y=0.
top_boss_z = sphere_cz + outer_r;

$fn = 80;

module shell_2d() {
    n = 50;
    outer = concat(
        [[collar_or, -collar_len]],
        [[outer_r,   0]],
        [for(i=[0:n]) [outer_r*cos(i*90/n), sphere_cz + outer_r*sin(i*90/n)]],
        [[0, sphere_cz + outer_r]]
    );
    inner = concat(
        [[0, sphere_cz + inner_r]],
        [for(i=[n:-1:0]) [inner_r*cos(i*90/n), sphere_cz + inner_r*sin(i*90/n)]],
        [[inner_r,   0]],
        [[collar_ir, -collar_len]]
    );
    polygon(concat(outer, inner));
}

module shell_solid_2d() {
    n = 50;
    polygon(concat(
        [[0, -collar_len], [collar_or, -collar_len], [outer_r, 0]],
        [for(i=[0:n]) [outer_r*cos(i*90/n), sphere_cz + outer_r*sin(i*90/n)]],
        [[0, sphere_cz + outer_r]]
    ));
}

module full_shell() { rotate_extrude($fn=80) shell_2d(); }
module solid_shell() { rotate_extrude($fn=80) shell_solid_2d(); }

module cuts() {
    translate([0, 0, sphere_cz + 10])
        rotate([0, 90, 0]) cylinder(h=200, r=12, center=true, $fn=40);
    difference() {
        translate([0,0,sphere_cz-2.5]) cylinder(h=5, r=outer_r+1.0, $fn=80);
        translate([0,0,sphere_cz-3.0]) cylinder(h=6, r=outer_r-0.8, $fn=80);
    }
}

// Side bosses - Half A: protrude in +Y, heat insert pocket
module half_A_bosses() {
    for (p = boss_xz) {
        difference() {
            translate([p[0], 0, p[1]])
                rotate([-90, 0, 0])
                    cylinder(r=boss_r, h=boss_h, $fn=36);
            translate([p[0], -0.1, p[1]])
                rotate([-90, 0, 0])
                    cylinder(r=insert_r, h=insert_depth + 0.1, $fn=24);
        }
    }
}

// Side bosses - Half B: protrude in -Y, clearance hole
module half_B_bosses() {
    for (p = boss_xz) {
        difference() {
            translate([p[0], 0, p[1]])
                rotate([90, 0, 0])
                    cylinder(r=boss_r, h=boss_h, $fn=36);
            translate([p[0], 0.1, p[1]])
                rotate([90, 0, 0])
                    cylinder(r=clearance_r, h=boss_h + 0.2, $fn=24);
        }
    }
}

// Top boss Half A: full cylinder protruding in +Y from dome apex, with heat insert pocket
module top_boss_A() {
    difference() {
        translate([0, 0, top_boss_z])
            rotate([-90, 0, 0])
                cylinder(r=boss_r, h=boss_h, $fn=36);
        translate([0, -0.1, top_boss_z])
            rotate([-90, 0, 0])
                cylinder(r=insert_r, h=insert_depth + 0.1, $fn=24);
    }
}

// Top boss Half B: full cylinder protruding in -Y from dome apex, with M3 clearance hole
module top_boss_B() {
    difference() {
        translate([0, 0, top_boss_z])
            rotate([90, 0, 0])
                cylinder(r=boss_r, h=boss_h, $fn=36);
        translate([0, 0.1, top_boss_z])
            rotate([90, 0, 0])
                cylinder(r=clearance_r, h=boss_h + 0.2, $fn=24);
    }
}

// Show both halves separated (preview only)
module half_A() {
    difference() {
        union() {
            intersection() {
                full_shell();
                translate([-200, 0, -collar_len-1])
                    cube([400, 200, collar_len+sphere_cz+outer_r+5]);
            }
            half_A_bosses();
            top_boss_A();
        }
        cuts();
    }
}

module half_B() {
    difference() {
        union() {
            intersection() {
                full_shell();
                translate([-200, -200, -collar_len-1])
                    cube([400, 200, collar_len+sphere_cz+outer_r+5]);
            }
            half_B_bosses();
            top_boss_B();
        }
        cuts();
    }
}

translate([-(outer_r + 20), 0, 0]) half_A();
translate([  outer_r + 20,  0, 0]) half_B();

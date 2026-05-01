// Example 1: Simple Cube
// cube([50, 50, 50]);

// Example 2: Rounded Cube
module rounded_cube(size, radius) {
    minkowski() {
        cube([size[0] - 2*radius, size[1] - 2*radius, size[2] - 2*radius], center = true);
        sphere(r = radius);
    }
}

// Example 3: Phone Stand
module phone_stand() {
    // Base
    cube([150, 100, 10], center = true);
    
    // Support
    translate([0, 40, 15])
    rotate([45, 0, 0])
    cube([150, 60, 10], center = true);
    
    // Grooves
    translate([0, 30, 20])
    cube([140, 3, 5], center = true);
}

// Example 4: Parametric Box with Lid
module parametric_box(width, depth, height, wall_thickness) {
    // Bottom
    difference() {
        cube([width, depth, height]);
        translate([wall_thickness, wall_thickness, wall_thickness])
        cube([width - 2*wall_thickness, depth - 2*wall_thickness, height]);
    }
    
    // Lip for lid
    translate([0, 0, height])
    cube([width, depth, wall_thickness]);
}

// Example 5: Gear
module gear(teeth, radius, thickness) {
    gear_tooth_height = radius * 0.2;
    tooth_angle = 360 / teeth;
    
    cylinder(h = thickness, r = radius, center = true);
    
    for (i = [0 : teeth - 1])
    rotate([0, 0, i * tooth_angle])
    translate([radius, 0, 0])
    cube([radius * 0.3, radius * 0.2, thickness], center = true);
}

// Example 6: Spiral Tower
module spiral_tower(height, radius, turns) {
    n_segments = turns * 36;
    segment_height = height / n_segments;
    
    for (i = [0 : n_segments - 1]) {
        angle = i * (360 / 36);
        translate([radius * cos(angle), radius * sin(angle), i * segment_height])
        cylinder(h = segment_height, r = 5);
    }
}

// Example 7: Cable Organizer
module cable_organizer() {
    // Base
    cube([100, 50, 5], center = true);
    
    // Dividers
    for (x = [-30, 0, 30]) {
        translate([x, 0, 10])
        cube([3, 50, 20], center = true);
    }
    
    // Cable slots
    for (y = [-15, 0, 15]) {
        translate([0, y, 15])
        cylinder(h = 10, r = 5, center = true);
    }
}

// Example 8: Hexagonal Nut
module hex_nut(size, height, hole_radius) {
    difference() {
        cylinder(h = height, r = size / 1.73, $fn = 6, center = true);
        cylinder(h = height + 1, r = hole_radius, center = true);
    }
}

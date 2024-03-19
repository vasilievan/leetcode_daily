/// # Minimum Number of Arrows to Burst Balloons
/// ## Arguments
/// * `points` - A vector with balloons representation.
///
/// There are some spherical balloons taped onto a flat wall that represents the XY-plane.
/// The balloons are represented as a 2D integer array points where points[i] = [x_start, x_end]
/// denotes a balloon whose horizontal diameter stretches between x_start and x_end. You do not know
/// the exact y-coordinates of the balloons.
/// Arrows can be shot up directly vertically (in the positive y-direction) from different points
/// along the x-axis. A balloon with xstart and xend is burst by an arrow shot at x
/// if xstart <= x <= xend. There is no limit to the number of arrows that can be shot. A shot arrow
/// keeps traveling up infinitely, bursting any balloons in its path.
///
/// Given the array points, return the minimum number of arrows that must be shot to burst all
/// balloons.

pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_by(|first, second| first[1].cmp(&second[1]));
    let mut arrow_count = 1;
    let mut overlap = points[0][1];
    for index in 0..points.len() {
        if overlap >= points[index][0] {
            continue
        }
        arrow_count += 1;
        overlap = points[index][1];
    }
    return arrow_count;
}
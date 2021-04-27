use textplots::{utils, Chart, Plot, Point, Shape};

fn main() {
    let data = [
        (0.0, 11.677842),
        (1.0, 9.36832),
        (2.0, 9.44862),
        (3.0, 8.933405),
        (4.0, 11.812017),
        (5.0, 9.225881),
        (6.0, 11.498754),
        (7.0, 8.048191),
        (8.0, 11.141933),
        (9.0, 8.908442),
        (10.0, 9.795515),
        (11.0, 9.115709),
        (12.0, 8.907767),
        (13.0, 11.153717),
        (14.0, 10.893343),
        (15.0, 9.299543),
        (16.0, 9.701611),
        (17.0, 10.5055685),
        (18.0, 9.29626),
        (19.0, 9.14646),
        (20.0, 8.892315),
        (21.0, 10.202528),
        (22.0, 9.776375),
        (23.0, 9.106522),
        (24.0, 10.017476),
        (25.0, 9.344163),
        (26.0, 10.087107),
        (27.0, 10.595626),
        (28.0, 10.26935),
        (29.0, 9.619124),
        (30.0, 9.759572),
        (31.0, 10.563857),
        (32.0, 10.254505),
        (33.0, 10.876704),
        (34.0, 10.487561),
        (35.0, 9.672023),
        (36.0, 11.139632),
        (37.0, 11.562245),
        (38.0, 12.311694),
        (39.0, 8.636497),
        (40.0, 10.127292),
        (41.0, 10.867212),
        (42.0, 10.285108),
        (43.0, 8.08108),
        (44.0, 10.198147),
        (45.0, 10.746926),
        (46.0, 9.746353),
        (47.0, 9.767584),
        (48.0, 10.165949),
        (49.0, 10.08595),
        (50.0, 10.08878),
        (51.0, 11.137325),
        (52.0, 9.596683),
        (53.0, 9.3425045),
        (54.0, 9.627313),
        (55.0, 10.010254),
        (56.0, 9.526915),
        (57.0, 11.239794),
        (58.0, 8.913055),
        (59.0, 10.136724),
        (60.0, 10.72442),
        (61.0, 9.044553),
        (62.0, 8.657727),
        (63.0, 10.284623),
        (64.0, 10.32074),
        (65.0, 8.713137),
        (66.0, 9.928085),
        (67.0, 8.439049),
        (68.0, 9.942111),
        (69.0, 9.212654),
        (70.0, 8.224474),
        (71.0, 11.252406),
        (72.0, 8.816701),
        (73.0, 10.853656),
        (74.0, 8.788404),
        (75.0, 10.526451),
        (76.0, 10.779287),
        (77.0, 9.357046),
        (78.0, 10.788815),
        (79.0, 10.013228),
        (80.0, 10.859512),
        (81.0, 10.734754),
        (82.0, 10.504648),
        (83.0, 10.104772),
        (84.0, 10.20058),
        (85.0, 10.663727),
        (86.0, 11.040146),
        (87.0, 12.313308),
        (88.0, 10.41382),
        (89.0, 9.867012),
        (90.0, 9.984057),
        (91.0, 8.8879595),
        (92.0, 9.459296),
        (93.0, 9.00407),
        (94.0, 10.469272),
        (95.0, 9.79327),
        (96.0, 12.317585),
        (97.0, 8.190812),
        (98.0, 12.709852),
        (99.0, 9.720502),
    ];

    let points: Vec<Point<f32, f32>> = utils::f32s_into_points(&data);
    println!("\ny = line plot");
    Chart::new(180, 60, 0.0, 100.0)
        .lineplot(&Shape::Lines(&points))
        .display();

    println!("\ny = steps");
    Chart::new(180, 60, 0.0, 100.0)
        .lineplot(&Shape::Steps(&points))
        .display();

    println!("\ny = bars");
    Chart::new(180, 60, 0.0, 100.0)
        .lineplot(&Shape::Bars(&points))
        .display();

    let hist = utils::histogram(&data, 6.0, 15.0, 16);
    println!("\ny = histogram bars");
    Chart::new(180, 60, 6.0, 14.0)
        .lineplot(&Shape::Bars(&hist))
        .nice();
}

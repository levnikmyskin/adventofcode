mod locations;
use crate::locations::Locations;

fn main() {
    let coordinates = [
        (80, 357, 0),
        (252, 184, 1),
        (187, 139, 2),
        (101, 247, 3),
        (332, 328, 4),
        (302, 60, 5),
        (196, 113, 6),
        (271, 201, 7),
        (334, 89, 8),
        (85, 139, 9),
        (327, 161, 10),
        (316, 352, 11),
        (343, 208, 12),
        (303, 325, 13),
        (316, 149, 14),
        (270, 319, 15),
        (318, 153, 16),
        (257, 332, 17),
        (306, 348, 18),
        (299, 358, 19),
        (172, 289, 20),
        (303, 349, 21),
        (271, 205, 22),
        (347, 296, 23),
        (220, 276, 24),
        (235, 231, 25),
        (133, 201, 26),
        (262, 355, 27),
        (72, 71, 28),
        (73, 145, 29),
        (310, 298, 31),
        (138, 244, 32),
        (322, 334, 33),
        (278, 148, 34),
        (126, 135, 35),
        (340, 133, 36),
        (311, 118, 37),
        (193, 173, 38),
        (319, 99, 39),
        (50, 309, 40),
        (160, 356, 41),
        (155, 195, 43),
        (61, 319, 44),
        (80, 259, 45),
        (106, 318, 46),
        (49, 169, 47),
        (134, 61, 48),
        (74, 204, 49),
        (337, 174, 50),
        (108, 287, 51),
    ];
    let test_coordinates = [
        (1, 1, 2),
        (1, 6, 3),
        (8, 3, 4),
        (3, 4, 5),
        (5, 5, 6),
        (8, 9, 7),
    ];

    let (max_x, _, _) = coordinates.iter().max_by_key(|(x, y, l)| x).unwrap();
    let (_, max_y, _) = coordinates.iter().max_by_key(|(x, y, l)| y).unwrap();

    let mut l = Locations::new(*max_y + 1, *max_x + 1, coordinates.len());
    for (x, y, label) in coordinates.iter() {
        l.insert_new_coordinates(*x, *y, *label);
    }

    println!("within 10000 distance region size is: {}",  l.less_than_n_distance_region(10000));
}


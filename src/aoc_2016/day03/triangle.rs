#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Triangle {}
impl Triangle {
    pub fn is_valid(mut sides: Vec<u32>) -> bool {
        if sides.len() != 3 {
            return false;
        }
        sides.sort();
        sides[0] + sides[1] > sides[2]
    }

    pub fn count_triangles(list_of_sides: Vec<Vec<u32>>) -> usize {
        list_of_sides
            .into_iter()
            .map(|sides| Triangle::is_valid(sides))
            .filter(|b| *b)
            .count()
    }
}

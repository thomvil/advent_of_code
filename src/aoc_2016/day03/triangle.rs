#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Triangle {}
impl Triangle {
    pub fn is_valid(sides: &[u32]) -> bool {
        let min = sides.iter().min().unwrap();
        let max = sides.iter().max().unwrap();
        let med = sides.iter().sum::<u32>() - min - max;
        sides.len() == 3 && (*min + med > *max)
    }

    pub fn count_triangles(list_of_sides: Vec<Vec<u32>>) -> usize {
        list_of_sides
            .into_iter()
            .filter(|sides| Triangle::is_valid(sides))
            .count()
    }
}

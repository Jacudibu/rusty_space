use crate::sectors::sector::AllSectors;
use hexx::Hex;

fn find_path(sectors: &AllSectors, from: Hex, to: Hex) -> Option<Vec<Hex>> {
    hexx::algorithms::a_star(from, to, |a, b| {
        if a == b {
            return Some(0);
        }

        let sector = sectors.get(&a)?;
        let target_gate = sector.gates.get(&b)?;

        // TODO: Distance between gates should affect cost, but sadly we lack information on our origin as long as we use the built-in a_star function
        //       Spinning our own would in general speed things up since 60%+ of all_neighbors gate connections don't exist here
        Some(1)
    })
}

#[cfg(test)]
mod test {
    use crate::sectors::pathfinding::find_path;
    use crate::sectors::sector::*;
    use bevy::prelude::Entity;
    use bevy::utils::HashMap;
    use hexx::Hex;

    fn add_sector(all_sectors: &mut AllSectors, pos: Hex, gates: Vec<(Hex, Entity)>) {
        all_sectors.insert(
            pos,
            SectorData {
                coordinate: pos,
                entity: Entity::from_raw(0),
                gates: HashMap::from_iter(gates),
                ships: Vec::new(),
                stations: Vec::new(),
            },
        );
    }

    #[test]
    fn find_path_to_neighbor() {
        let mut all_sectors = AllSectors::default();
        let from = Hex::new(0, 0);
        let to = Hex::new(1, 0);

        let mock_entity = Entity::from_raw(0);

        add_sector(&mut all_sectors, from, vec![(to, mock_entity)]);
        add_sector(&mut all_sectors, to, vec![(from, mock_entity)]);

        let result = find_path(&all_sectors, from, to).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], from);
        assert_eq!(result[1], to);
    }

    #[test]
    fn find_path_across_corner() {
        let mut all_sectors = AllSectors::default();
        let center = Hex::ZERO;
        let right = Hex::new(1, 0);
        let top_right = Hex::new(0, 1);

        let mock_entity = Entity::from_raw(0);

        add_sector(&mut all_sectors, center, vec![(right, mock_entity)]);
        add_sector(
            &mut all_sectors,
            right,
            vec![(center, mock_entity), (top_right, mock_entity)],
        );
        add_sector(&mut all_sectors, top_right, vec![(right, mock_entity)]);

        let result = find_path(&all_sectors, center, top_right).unwrap();

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], center);
        assert_eq!(result[1], right);
        assert_eq!(result[2], top_right);
    }
}

//Er diagram connects entities with a name and attributes beetwen them
//the relationships can be 1 to N, N to N

use core::fmt;
use std::collections::HashMap;
#[derive(Debug, PartialEq)]
pub enum RelationshipType {
    One,
    Many,
}

#[derive(Debug, PartialEq)]
pub struct Attribute {
    name: String,
    primary_key: bool,
}
impl Attribute {
    pub fn new(name: &str, primary_key: bool) -> Self {
        Self {
            name: name.to_owned(),
            primary_key,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn is_primary_key(&self) -> bool {
        self.primary_key
    }
}
impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.primary_key {
            write!(f, "{}{{pk}}", self.name)?;
        } else {
            write!(f, "{}", self.name)?;
        }
        Ok(())
    }
}
#[derive(Debug, PartialEq)]
pub struct Entity {
    id: u32,
    name: String,
    attributes: Vec<Attribute>,
}
impl Entity {
    fn new(id: u32, name: &str) -> Self {
        Entity {
            id,
            name: name.to_owned(),
            attributes: Vec::new(),
        }
    }
    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn add_attribute(&mut self, attr: &str, pk: bool) -> Result<(), String> {
        if self.attributes.iter().any(|el| el.name == attr) {
            return Err(format!(
                "attribute {attr} already present in entity {}",
                self.name
            ));
        };
        if pk && self.attributes.iter().any(|el| el.primary_key) {
            return Err(format!(
                "primary key already present in entity {}",
                self.name
            ));
        }

        if pk {
            self.attributes.insert(0, Attribute::new(attr, pk));
        } else {
            self.attributes.push(Attribute::new(attr, pk));
        }
        Ok(())
    }
    pub fn remove_attr(&mut self, attr_name: &str) -> Result<(), String> {
        let Some((i, _)) = self
            .attributes
            .iter()
            .enumerate()
            .find(|(_, val)| val.name == attr_name)
        else {
            return Err(format!(
                "No attr '{attr_name}' found for entity {}",
                self.name
            ));
        };
        self.attributes.remove(i);
        Ok(())
    }
    pub fn mod_attr(&mut self, attr_name: &str, new_name: &str) -> Result<(), String> {
        let Some((i, val)) = self
            .attributes
            .iter()
            .enumerate()
            .find(|(_, val)| val.name == attr_name)
        else {
            return Err(format!(
                "No attr '{attr_name}' found for entity {}",
                self.name
            ));
        };
        if val.name == new_name {
            return Ok(());
        }
        let None = self.attributes.iter().find(|val| val.name == new_name) else {
            return Err(format!(
                "attribute '{new_name}' already present cannot reassign {}",
                self.name
            ));
        };
        self.attributes[i].name = new_name.to_owned();
        Ok(())
    }
    pub fn set_pk(&mut self, attr_name: &str) -> Result<(), String> {
        let Some((i, _)) = self
            .attributes
            .iter()
            .enumerate()
            .find(|(_, val)| val.name == attr_name)
        else {
            return Err(format!(
                "No attr '{attr_name}' found for entity {}",
                self.name
            ));
        };
        if let Some(pk_attr) = self.attributes.iter_mut().find(|val| val.primary_key) {
            pk_attr.primary_key = false;
        }
        self.attributes[i].primary_key = true;
        self.move_attr_index(i, 0)
            .expect("wouldn't error, if it did there are biiiiggg problems");
        Ok(())
    }
    pub fn set_attribute_pos(&mut self, attr_name: &str, new_pos: usize) -> Result<(), String> {
        let has_pks = self.attributes.iter().any(|el| el.primary_key);

        let is_pk = self
            .attributes
            .iter()
            .find(|el| el.name == attr_name && el.primary_key)
            .is_some();
        let exists = self
            .attributes
            .iter()
            .enumerate()
            .find(|(_, el)| el.name == attr_name);
        let Some((cur_pos, _)) = exists else {
            return Err(format!(
                "attribute {attr_name} not found in entity {}",
                self.name
            ));
        };
        if new_pos == cur_pos {
            return Ok(());
        }; //same position movement is always consented
        if (new_pos == 0 && has_pks) || is_pk {
            return Err("first position reserved for primary keys".to_owned());
        }
        self.move_attr_index(cur_pos, new_pos)
    }
    fn move_attr_index(&mut self, from: usize, to: usize) -> Result<(), String> {
        if from >= self.attributes.len() || to >= self.attributes.len() {
            return Err("index out of bounds".to_owned());
        }
        if from == to {
            return Ok(());
        }
        let element = self.attributes.remove(from);
        self.attributes.insert(to, element);
        Ok(())
    }
}
#[derive(Debug, PartialEq)]
pub struct Relationship {
    from_id: u32,
    to_id: u32,
    pub from_type: RelationshipType,
    pub to_type: RelationshipType,
}
#[derive(Debug, PartialEq)]
pub struct EntityGraph {
    id_counter: u32,
    entity_ids: HashMap<String, u32>,
    entities: HashMap<u32, Entity>,
    relationships: Vec<Relationship>,
}
impl EntityGraph {
    pub fn new() -> Self {
        EntityGraph {
            id_counter: 0,
            entity_ids: HashMap::new(),
            entities: HashMap::new(),
            relationships: Vec::new(),
        }
    }
    pub fn entities(&self) -> &HashMap<u32, Entity> {
        &self.entities
    }
    pub fn relationships(&self) -> &Vec<Relationship> {
        &self.relationships
    }
    fn entity_exists(&self, name: &str) -> bool {
        self.entity_ids.contains_key(name)
    }
    fn rel_index(&self, a: u32, b: u32) -> Option<usize> {
        self.relationships
            .iter()
            .position(|r| (r.from_id == a && r.to_id == b) || (r.from_id == b && r.to_id == a))
    }
    pub fn get_entity_relationships(&self, entity: &str) -> Option<Vec<&Relationship>> {
        let id = self.entity_ids.get(entity)?;
        let id = *id;
        let entity_rel: Vec<&Relationship> = self
            .relationships
            .iter()
            .filter(|el| el.from_id == id || el.to_id == id)
            .collect();
        Some(entity_rel)
    }
    pub fn add_entity(
        &mut self,
        name: &str,
        attributes: Option<&[Attribute]>,
    ) -> Result<(), String> {
        if self.entity_exists(name) {
            return Err(format!("entity {name} already in the graph"));
        }

        let mut new_entity = Entity::new(self.id_counter, name);
        if let Some(attr) = attributes {
            for item in attr {
                new_entity.add_attribute(&item.name, item.primary_key)?;
            }
        }

        self.id_counter += 1;
        self.entity_ids.insert(name.to_owned(), new_entity.id);
        self.entities.insert(new_entity.id, new_entity);
        Ok(())
    }
    pub fn mod_entity(&mut self, name: &str, new_name: &str) -> Result<(), String> {
        let mod_ent = match self.get_entity(name) {
            Some(ent) => ent,
            None => return Err(format!("entity {name} not found in graph")),
        };
        let old_name = mod_ent.name.to_owned();
        let id = mod_ent.id;
        if old_name == new_name {
            return Ok(());
        }
        let None = self.get_entity(new_name) else {
            return Err(format!("entity {new_name} already in graph, cannot modify"));
        };
        self.entity_ids
            .remove(&old_name)
            .expect("id not found even if entity in graph?");
        self.entity_ids.insert(new_name.to_owned(), id);
        self.entities.get_mut(&id).unwrap().name = new_name.to_owned();
        Ok(())
    }
    pub fn rem_entity(&mut self, name: &str) -> Result<(), String> {
        let Some(rem_ent) = self.get_entity(name) else {
            return Err(format!("entity {name} not found in graph"));
        };
        let rem_name = rem_ent.name.to_owned();

        let id = self
            .entity_ids
            .remove(&rem_name)
            .expect("id not found even if entity in graph");
        self.relationships
            .retain(|el| el.from_id != id && el.to_id != id);
        self.entities.remove(&id);

        Ok(())
    }
    pub fn add_relationship(
        &mut self,
        from: &str,
        from_type: RelationshipType,
        to: &str,
        to_type: RelationshipType,
    ) -> Result<(), String> {
        let from_id = match self.get_entity(from) {
            Some(val) => val.id,
            None => return Err("from entity not in graph".to_owned()),
        };
        let to_id = match self.get_entity(to) {
            Some(val) => val.id,
            None => return Err("to entity not in graph".to_owned()),
        };
        if self.rel_index(from_id, to_id).is_some() {
            return Err("relationship already exists".to_owned());
        };
        self.relationships.push(Relationship {
            from_id,
            to_id,
            from_type,
            to_type,
        });
        Ok(())
    }
    pub fn remove_relationship(&mut self, from: &str, to: &str) -> Result<(), String> {
        let from_id = match self.get_entity(from) {
            Some(val) => val.id,
            None => return Err("from entity not in graph".to_owned()),
        };
        let to_id = match self.get_entity(to) {
            Some(val) => val.id,
            None => return Err("to entity not in graph".to_owned()),
        };
        let Some(i) = self.rel_index(from_id, to_id) else {
            return Err(format!("relationship from {from} to {to} not found"));
        };
        self.relationships.remove(i);
        Ok(())
    }
    pub fn get_entity(&self, name: &str) -> Option<&Entity> {
        let id = self.entity_ids.get(name)?;
        self.entities.get(id)
    }
    pub fn get_entity_id(&self, name: &str) -> Option<u32> {
        let id = self.entity_ids.get(name)?;
        Some(*id)
    }
}
impl Default for EntityGraph {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod test {

    use super::*;
    mod entity_test {
        use super::super::*;
        #[test]
        fn entity_add_attr_success_index_0_pk_check() {
            //checks if adding functionality and pk priority at index 0 works
            let mut c = Entity::new(0, "test add success");
            c.add_attribute("test_attr", false).unwrap();
            c.add_attribute("test but pk", true).unwrap();
            let mut a = Entity::new(1, "inverse test add success");
            a.add_attribute("test but pk", true).unwrap();
            a.add_attribute("test_attr", false).unwrap();

            println!("first test");
            assert_eq!(c.attributes[0].name, "test but pk");
            assert_eq!(c.attributes[1].name, "test_attr");
            println!("inverse test");
            assert_eq!(a.attributes[0].name, "test but pk");
            assert_eq!(a.attributes[1].name, "test_attr");
        }
        #[test]
        fn entity_add_attr_fail_same_name() {
            let mut c = Entity::new(0, "test add fail");
            c.add_attribute("test_attr", false)
                .unwrap_or_else(|e| panic!("add_entity errored out before it needed to with {e}"));
            assert!(c.add_attribute("test_attr", false).is_err());
            assert_eq!(c.attributes[0].name, "test_attr");
            assert!(c.attributes.get(1).is_none());
        }
        #[test]
        fn entity_add_attr_fail_pk_present() {
            println!("testing with primary key disabled for second element");
            let mut c = Entity::new(0, "test add fail");
            c.add_attribute("pk_1", true).unwrap();
            assert!(c.add_attribute("pk_2", true).is_err());
            assert_eq!(c.attributes[0].name, "pk_1");
            assert!(c.attributes.get(1).is_none());
        }
        #[test]
        fn entity_remove_attr_success() {
            let mut a = Entity::new(0, "test remove success");
            a.add_attribute("test no pk", false).unwrap();
            a.remove_attr("test no pk").unwrap();
            assert!(a.attributes.get(0).is_none());
        }
        #[test]
        fn entity_remove_attr_fail() {
            let mut a = Entity::new(0, "test remove fail");
            a.add_attribute("my_attr", false).unwrap();
            assert!(a.remove_attr("test").is_err());
            assert!(a.attributes.get(0).is_some());
        }
        #[test]
        fn entity_mod_attr_success() {
            let mut a = Entity::new(0, "test mod success");
            a.add_attribute("test no pk", false).unwrap();
            a.mod_attr("test no pk", "test but new").unwrap();
            assert_eq!(a.attributes[0].name, "test but new");
        }
        #[test]
        fn entity_mod_attr_success_same_name() {
            let mut a = Entity::new(0, "test mod success");
            a.add_attribute("test no pk", false).unwrap();
            a.mod_attr("test no pk", "test no pk").unwrap();
            assert_eq!(a.attributes[0].name, "test no pk");
        }
        #[test]
        fn entity_mod_attr_fail_no_attr() {
            let mut a = Entity::new(0, "test mod fail");
            a.add_attribute("name", false).unwrap();
            assert!(a.mod_attr("test", "uninmportant").is_err());
            assert_eq!(a.attributes[0].name, "name");
        }
        #[test]
        fn entity_mod_attr_fail_same_name() {
            let mut a = Entity::new(0, "test mod fail");
            a.add_attribute("test", false).unwrap();
            a.add_attribute("test_new", false).unwrap();
            assert!(a.mod_attr("test", "test_new").is_err());
            assert_eq!(a.attributes[0].name, "test");
            assert_eq!(a.attributes[1].name, "test_new");
        }
        #[test]
        fn entity_set_pk_attr_success() {
            println!("testing with no pk present");
            let mut dummy = Entity::new(0, "test set pk");
            dummy.add_attribute("test1", false).unwrap();
            dummy.add_attribute("test2", false).unwrap();
            dummy.add_attribute("test3", false).unwrap();
            dummy.set_pk("test3").unwrap();
            assert_eq!(dummy.attributes[0].name, "test3");
            assert!(dummy.attributes[0].primary_key);
            assert!(!dummy.attributes[1].primary_key && !dummy.attributes[2].primary_key);
        }
        #[test]
        fn entity_set_pk_attr_success_pk_present() {
            let mut dummy = Entity::new(0, "test set pk");
            dummy.add_attribute("test1", true).unwrap();
            dummy.add_attribute("test2", false).unwrap();
            dummy.add_attribute("test3", false).unwrap();
            dummy.set_pk("test3").unwrap();
            assert_eq!(dummy.attributes[0].name, "test3");
            assert!(dummy.attributes[0].primary_key);
            assert!(!dummy.attributes[1].primary_key && !dummy.attributes[2].primary_key);
        }
        #[test]
        fn entity_set_pk_attr_fail() {
            let mut a = Entity::new(0, "test set_pk fail");
            a.add_attribute("test", false).unwrap();
            a.add_attribute("test2", false).unwrap();
            assert!(a.set_pk("foo").is_err());
            assert!(!a.attributes[0].primary_key && !a.attributes[1].primary_key);
            assert_eq!(a.attributes[0].name, "test");
            assert_eq!(a.attributes[1].name, "test2");
        }
        #[test]
        fn entity_set_attr_pos_success() {
            let mut dummy = Entity::new(0, "test set_attr_pos success");
            dummy.add_attribute("test1", false).unwrap();
            dummy.add_attribute("test2", false).unwrap();
            dummy.add_attribute("test3", false).unwrap();
            dummy.set_attribute_pos("test3", 0).unwrap();
            //3 1 2
            dummy.set_attribute_pos("test2", 1).unwrap();
            //3 2 1
            assert_eq!(dummy.attributes[0].name, "test3");
            assert_eq!(dummy.attributes[1].name, "test2");
            assert_eq!(dummy.attributes[2].name, "test1");
        }
        #[test]
        fn entity_set_same_pos_success() {
            let mut dummy = Entity::new(0, "test set_attr_pos success");
            dummy.add_attribute("test1", true).unwrap();
            dummy.add_attribute("test2", false).unwrap();
            dummy.add_attribute("test3", false).unwrap();
            dummy.set_attribute_pos("test1", 0).unwrap();
            assert_eq!(dummy.attributes[0].name, "test1");
            assert_eq!(dummy.attributes[1].name, "test2");
            assert_eq!(dummy.attributes[2].name, "test3");
        }
        #[test]
        fn entity_set_fail_primary_key() {
            let mut dummy = Entity::new(0, "test set_attr_pos success");
            dummy.add_attribute("test1", true).unwrap();
            dummy.add_attribute("test2", false).unwrap();
            dummy.add_attribute("test3", false).unwrap();
            assert!(dummy.set_attribute_pos("test1", 2).is_err());
            assert_eq!(dummy.attributes[0].name, "test1");
            assert_eq!(dummy.attributes[1].name, "test2");
            assert_eq!(dummy.attributes[2].name, "test3");
        }
        #[test]
        fn entity_set_fail_reserved_pos() {
            let mut dummy = Entity::new(0, "test set_attr_pos fail");
            dummy.add_attribute("test1", true).unwrap();
            dummy.add_attribute("test2", false).unwrap();
            dummy.add_attribute("test3", false).unwrap();
            assert!(dummy.set_attribute_pos("test3", 0).is_err());
            assert_eq!(dummy.attributes[0].name, "test1");
            assert_eq!(dummy.attributes[1].name, "test2");
            assert_eq!(dummy.attributes[2].name, "test3");
        }
        #[test]
        fn entity_set_fail_unknown_attr() {
            let mut dummy = Entity::new(0, "test set_attr_pos fail");
            dummy.add_attribute("test1", false).unwrap();
            dummy.add_attribute("test2", false).unwrap();
            dummy.add_attribute("test3", false).unwrap();
            assert!(dummy.set_attribute_pos("foo", 0).is_err());
            assert_eq!(dummy.attributes[0].name, "test1");
            assert_eq!(dummy.attributes[1].name, "test2");
            assert_eq!(dummy.attributes[2].name, "test3");
        }
        #[test]
        fn entity_set_fail_out_of_bounds() {
            let mut dummy = Entity::new(0, "test set_attr_pos fail");
            dummy.add_attribute("test1", false).unwrap();
            dummy.add_attribute("test2", false).unwrap();
            dummy.add_attribute("test3", false).unwrap();
            assert!(dummy.set_attribute_pos("test1", 100).is_err());
            assert_eq!(dummy.attributes[0].name, "test1");
            assert_eq!(dummy.attributes[1].name, "test2");
            assert_eq!(dummy.attributes[2].name, "test3");
        }
    }

    #[test]
    fn graph_add_entity_success() {
        let mut dummy = EntityGraph::new();
        let dummy_attrs = [
            Attribute {
                name: "test_attr".to_owned(),
                primary_key: false,
            },
            Attribute {
                name: "test_attr2".to_owned(),
                primary_key: true,
            },
        ];
        dummy.add_entity("test1", Some(&dummy_attrs)).unwrap();
        dummy.add_entity("test2", None).unwrap();
        assert!(dummy.get_entity("test1").is_some());
        assert!(dummy.get_entity("test2").is_some());
    }
    #[test]
    fn graph_add_entity_fail_same_name() {
        let mut dummy = EntityGraph::new();
        let dummy_attrs = [Attribute {
            name: "test".to_owned(),
            primary_key: false,
        }];
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", Some(&dummy_attrs)).unwrap();
        assert!(dummy.add_entity("test2", None).is_err());
        let ent = dummy.get_entity("test2").unwrap();
        assert_eq!(ent.attributes, dummy_attrs);
    }
    #[test]
    fn graph_add_entity_fail_inv_attrs() {
        let mut dummy = EntityGraph::new();
        let dummy_attrs = [
            Attribute {
                name: "test_attr".to_owned(),
                primary_key: true,
            },
            Attribute {
                name: "test_attr2".to_owned(),
                primary_key: true,
            },
        ];
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", None).unwrap();
        assert!(dummy.add_entity("test3", Some(&dummy_attrs)).is_err());
        assert!(dummy.get_entity("test3").is_none());
    }

    #[test]
    fn graph_add_rel_one_one_success() {
        let mut dummy = EntityGraph::new();
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", None).unwrap();
        dummy.add_entity("test3", None).unwrap();
        dummy
            .add_relationship(
                "test1",
                RelationshipType::One,
                "test2",
                RelationshipType::One,
            )
            .unwrap();
        let from_id = dummy.get_entity_id("test1").unwrap();
        let to_id = dummy.get_entity_id("test2").unwrap();
        let one_to_one = Relationship {
            from_id,
            to_id,
            from_type: RelationshipType::One,
            to_type: RelationshipType::One,
        };

        assert!(dummy.get_entity_relationships("test3").unwrap().is_empty());
        assert!(
            dummy
                .get_entity_relationships("test1")
                .unwrap()
                .contains(&&one_to_one)
        );
        assert!(
            dummy
                .get_entity_relationships("test2")
                .unwrap()
                .contains(&&one_to_one)
        );
    }
    #[test]
    fn graph_add_rel_fail_from_entity() {
        let mut dummy = EntityGraph::new();
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", None).unwrap();
        dummy.add_entity("test3", None).unwrap();
        dummy
            .add_relationship(
                "test1",
                RelationshipType::One,
                "test2",
                RelationshipType::One,
            )
            .unwrap();
        let from_id = dummy.get_entity_id("test1").unwrap();
        let to_id = dummy.get_entity_id("test2").unwrap();
        let one_to_one = Relationship {
            from_id,
            to_id,
            from_type: RelationshipType::One,
            to_type: RelationshipType::One,
        };

        assert!(
            dummy
                .add_relationship("foo", RelationshipType::One, "test3", RelationshipType::One)
                .is_err()
        );
        assert!(dummy.get_entity_relationships("test3").unwrap().is_empty());
        assert!(
            dummy
                .get_entity_relationships("test1")
                .unwrap()
                .contains(&&one_to_one)
        );
        assert!(
            dummy
                .get_entity_relationships("test2")
                .unwrap()
                .contains(&&one_to_one)
        );
    }
    #[test]
    fn graph_add_rel_fail_to_entity() {
        let mut dummy = EntityGraph::new();
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", None).unwrap();
        dummy.add_entity("test3", None).unwrap();
        dummy
            .add_relationship(
                "test1",
                RelationshipType::One,
                "test2",
                RelationshipType::One,
            )
            .unwrap();
        let from_id = dummy.get_entity_id("test1").unwrap();
        let to_id = dummy.get_entity_id("test2").unwrap();
        let one_to_one = Relationship {
            from_id,
            to_id,
            from_type: RelationshipType::One,
            to_type: RelationshipType::One,
        };

        assert!(
            dummy
                .add_relationship("test3", RelationshipType::One, "foo", RelationshipType::One)
                .is_err()
        );
        assert!(dummy.get_entity_relationships("test3").unwrap().is_empty());
        assert!(
            dummy
                .get_entity_relationships("test1")
                .unwrap()
                .contains(&&one_to_one)
        );
        assert!(
            dummy
                .get_entity_relationships("test2")
                .unwrap()
                .contains(&&one_to_one)
        );
    }
    #[test]
    fn graph_add_rel_fail_same_rel_entity() {
        let mut dummy = EntityGraph::new();
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", None).unwrap();
        dummy.add_entity("test3", None).unwrap();
        dummy
            .add_relationship(
                "test1",
                RelationshipType::One,
                "test2",
                RelationshipType::One,
            )
            .unwrap();
        let from_id = dummy.get_entity_id("test1").unwrap();
        let to_id = dummy.get_entity_id("test2").unwrap();
        let one_to_one = Relationship {
            from_id,
            to_id,
            from_type: RelationshipType::One,
            to_type: RelationshipType::One,
        };

        assert!(
            dummy
                .add_relationship(
                    "test1",
                    RelationshipType::One,
                    "test2",
                    RelationshipType::One
                )
                .is_err()
        );
        assert!(dummy.get_entity_relationships("test3").unwrap().is_empty());
        assert!(
            dummy
                .get_entity_relationships("test1")
                .unwrap()
                .contains(&&one_to_one)
        );
        assert!(
            dummy
                .get_entity_relationships("test2")
                .unwrap()
                .contains(&&one_to_one)
        );
    }
    #[test]
    fn graph_rem_entity_first_success() {
        let mut dummy = EntityGraph::new();
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", None).unwrap();
        dummy.add_entity("test3", None).unwrap();
        dummy
            .add_relationship(
                "test1",
                RelationshipType::One,
                "test2",
                RelationshipType::Many,
            )
            .unwrap();
        dummy
            .add_relationship(
                "test1",
                RelationshipType::One,
                "test3",
                RelationshipType::Many,
            )
            .unwrap();
        dummy.rem_entity("test1").unwrap();
        assert!(dummy.get_entity("test1").is_none());
        assert!(dummy.get_entity_relationships("test1").is_none());
        assert!(dummy.get_entity_relationships("test2").unwrap().is_empty());
        assert!(dummy.get_entity_relationships("test3").unwrap().is_empty());
        assert_eq!(dummy.get_entity("test2").unwrap().name, "test2");
        assert_eq!(dummy.get_entity("test3").unwrap().name, "test3");
    }
    #[test]
    fn graph_rem_entity_last_success() {
        let mut dummy = EntityGraph::new();
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", None).unwrap();
        dummy.add_entity("test3", None).unwrap();
        dummy
            .add_relationship(
                "test3",
                RelationshipType::One,
                "test2",
                RelationshipType::Many,
            )
            .unwrap();
        dummy
            .add_relationship(
                "test3",
                RelationshipType::One,
                "test1",
                RelationshipType::Many,
            )
            .unwrap();
        dummy.rem_entity("test3").unwrap();
        assert!(dummy.get_entity("test3").is_none());
        assert!(dummy.get_entity_relationships("test3").is_none());
        assert!(dummy.get_entity_relationships("test1").unwrap().is_empty());
        assert!(dummy.get_entity_relationships("test2").unwrap().is_empty());
        assert_eq!(dummy.get_entity("test1").unwrap().name, "test1");
        assert_eq!(dummy.get_entity("test2").unwrap().name, "test2");
    }
    #[test]
    fn graph_rem_entity_beetwen_success() {
        let mut dummy = EntityGraph::new();
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", None).unwrap();
        dummy.add_entity("test3", None).unwrap();
        dummy
            .add_relationship(
                "test2",
                RelationshipType::One,
                "test1",
                RelationshipType::Many,
            )
            .unwrap();
        dummy
            .add_relationship(
                "test2",
                RelationshipType::One,
                "test3",
                RelationshipType::Many,
            )
            .unwrap();
        dummy.rem_entity("test2").unwrap();
        assert!(dummy.get_entity("test2").is_none());
        assert!(dummy.get_entity_relationships("test2").is_none());
        assert!(dummy.get_entity_relationships("test1").unwrap().is_empty());
        assert!(dummy.get_entity_relationships("test3").unwrap().is_empty());
        assert_eq!(dummy.get_entity("test1").unwrap().name, "test1");
        assert_eq!(dummy.get_entity("test3").unwrap().name, "test3");
    }
    #[test]
    fn graph_rem_entity_fail_unknown_entity() {
        let mut dummy = EntityGraph::new();
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", None).unwrap();
        dummy.add_entity("test3", None).unwrap();
        dummy
            .add_relationship(
                "test1",
                RelationshipType::One,
                "test2",
                RelationshipType::Many,
            )
            .unwrap();
        dummy
            .add_relationship(
                "test2",
                RelationshipType::One,
                "test3",
                RelationshipType::Many,
            )
            .unwrap();
        assert!(dummy.rem_entity("foo").is_err());
        assert!(dummy.get_entity_relationships("test1").is_some());
        assert!(dummy.get_entity_relationships("test2").is_some());
        assert!(dummy.get_entity_relationships("test3").is_some());
        assert_eq!(dummy.get_entity("test1").unwrap().name, "test1");
        assert_eq!(dummy.get_entity("test2").unwrap().name, "test2");
        assert_eq!(dummy.get_entity("test3").unwrap().name, "test3");
    }
    #[test]
    fn graph_mod_name_success() {
        let mut dummy = EntityGraph::new();
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", None).unwrap();
        dummy.add_entity("test3", None).unwrap();
        let (bef_id1, bef_id2, bef_id3) = (
            dummy.get_entity_id("test1").unwrap(),
            dummy.get_entity_id("test2").unwrap(),
            dummy.get_entity_id("test3").unwrap(),
        );
        dummy.mod_entity("test1", "foo1").unwrap();
        assert!(dummy.get_entity("test1").is_none());
        assert!(dummy.get_entity("foo1").is_some());
        let (af_id1, af_id2, af_id3) = (
            dummy.get_entity_id("foo1").unwrap(),
            dummy.get_entity_id("test2").unwrap(),
            dummy.get_entity_id("test3").unwrap(),
        );
        assert_eq!(af_id1, bef_id1);
        assert_eq!(af_id2, bef_id2);
        assert_eq!(af_id3, bef_id3);
        let (af_name_1, af_name_2, af_name_3) = (
            &dummy.get_entity("foo1").unwrap().name,
            &dummy.get_entity("test2").unwrap().name,
            &dummy.get_entity("test3").unwrap().name,
        );
        assert_eq!(af_name_1, "foo1");
        assert_eq!(af_name_2, "test2");
        assert_eq!(af_name_3, "test3");
    }
    #[test]
    fn graph_mod_same_name_success() {
        let mut dummy = EntityGraph::new();
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", None).unwrap();
        dummy.add_entity("test3", None).unwrap();
        let (bef_id1, bef_id2, bef_id3) = (
            dummy.get_entity_id("test1").unwrap(),
            dummy.get_entity_id("test2").unwrap(),
            dummy.get_entity_id("test3").unwrap(),
        );
        dummy.mod_entity("test1", "test1").unwrap();
        assert!(dummy.get_entity("test1").is_some());
        let (af_id1, af_id2, af_id3) = (
            dummy.get_entity_id("test1").unwrap(),
            dummy.get_entity_id("test2").unwrap(),
            dummy.get_entity_id("test3").unwrap(),
        );
        assert_eq!(af_id1, bef_id1);
        assert_eq!(af_id2, bef_id2);
        assert_eq!(af_id3, bef_id3);
        let (af_name_1, af_name_2, af_name_3) = (
            &dummy.get_entity("test1").unwrap().name,
            &dummy.get_entity("test2").unwrap().name,
            &dummy.get_entity("test3").unwrap().name,
        );
        assert_eq!(af_name_1, "test1");
        assert_eq!(af_name_2, "test2");
        assert_eq!(af_name_3, "test3");
    }
    #[test]
    fn graph_mod_name_fail_unknown_entity() {
        let mut dummy = EntityGraph::new();
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", None).unwrap();
        dummy.add_entity("test3", None).unwrap();
        let (bef_id1, bef_id2, bef_id3) = (
            dummy.get_entity_id("test1").unwrap(),
            dummy.get_entity_id("test2").unwrap(),
            dummy.get_entity_id("test3").unwrap(),
        );
        assert!(dummy.mod_entity("test100", "foo1").is_err());
        assert!(dummy.get_entity("foo1").is_none());
        let (af_id1, af_id2, af_id3) = (
            dummy.get_entity_id("test1").unwrap(),
            dummy.get_entity_id("test2").unwrap(),
            dummy.get_entity_id("test3").unwrap(),
        );
        assert_eq!(af_id1, bef_id1);
        assert_eq!(af_id2, bef_id2);
        assert_eq!(af_id3, bef_id3);
        let (af_name_1, af_name_2, af_name_3) = (
            &dummy.get_entity("test1").unwrap().name,
            &dummy.get_entity("test2").unwrap().name,
            &dummy.get_entity("test3").unwrap().name,
        );
        assert_eq!(af_name_1, "test1");
        assert_eq!(af_name_2, "test2");
        assert_eq!(af_name_3, "test3");
    }
    #[test]
    fn graph_mod_name_fail_repeated_name() {
        let mut dummy = EntityGraph::new();
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", None).unwrap();
        dummy.add_entity("test3", None).unwrap();
        let (bef_id1, bef_id2, bef_id3) = (
            dummy.get_entity_id("test1").unwrap(),
            dummy.get_entity_id("test2").unwrap(),
            dummy.get_entity_id("test3").unwrap(),
        );
        assert!(dummy.mod_entity("test1", "test2").is_err());
        let (af_id1, af_id2, af_id3) = (
            dummy.get_entity_id("test1").unwrap(),
            dummy.get_entity_id("test2").unwrap(),
            dummy.get_entity_id("test3").unwrap(),
        );
        assert_eq!(af_id1, bef_id1);
        assert_eq!(af_id2, bef_id2);
        assert_eq!(af_id3, bef_id3);
        let (af_name_1, af_name_2, af_name_3) = (
            &dummy.get_entity("test1").unwrap().name,
            &dummy.get_entity("test2").unwrap().name,
            &dummy.get_entity("test3").unwrap().name,
        );
        assert_eq!(af_name_1, "test1");
        assert_eq!(af_name_2, "test2");
        assert_eq!(af_name_3, "test3");
    }
    #[test]
    fn graph_rem_rel_success() {
        let mut dummy = EntityGraph::new();
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", None).unwrap();
        dummy.add_entity("test3", None).unwrap();

        dummy
            .add_relationship(
                "test1",
                RelationshipType::One,
                "test2",
                RelationshipType::One,
            )
            .unwrap();
        dummy.remove_relationship("test1", "test2").unwrap();
        assert!(dummy.get_entity_relationships("test1").unwrap().is_empty());
        assert!(dummy.get_entity_relationships("test2").unwrap().is_empty());
    }
    #[test]
    fn graph_rem_rel_success_inverse() {
        let mut dummy = EntityGraph::new();
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", None).unwrap();
        dummy.add_entity("test3", None).unwrap();

        dummy
            .add_relationship(
                "test1",
                RelationshipType::One,
                "test2",
                RelationshipType::One,
            )
            .unwrap();
        dummy.remove_relationship("test2", "test1").unwrap();
        assert!(dummy.get_entity_relationships("test1").unwrap().is_empty());
        assert!(dummy.get_entity_relationships("test2").unwrap().is_empty());
    }
    #[test]
    fn graph_rem_rel_fail_no_from_entity() {
        let mut dummy = EntityGraph::new();
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", None).unwrap();
        dummy.add_entity("test3", None).unwrap();

        dummy
            .add_relationship(
                "test1",
                RelationshipType::One,
                "test2",
                RelationshipType::One,
            )
            .unwrap();
        assert!(dummy.remove_relationship("test10", "test1").is_err());
        assert_eq!(dummy.get_entity_relationships("test1").unwrap().len(), 1);
        assert_eq!(dummy.get_entity_relationships("test2").unwrap().len(), 1);
    }
    #[test]
    fn graph_rem_rel_fail_no_to_entity() {
        let mut dummy = EntityGraph::new();
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", None).unwrap();
        dummy.add_entity("test3", None).unwrap();

        dummy
            .add_relationship(
                "test1",
                RelationshipType::One,
                "test2",
                RelationshipType::One,
            )
            .unwrap();
        assert!(dummy.remove_relationship("test1", "test10").is_err());
        assert_eq!(dummy.get_entity_relationships("test1").unwrap().len(), 1);
        assert_eq!(dummy.get_entity_relationships("test2").unwrap().len(), 1);
    }
    #[test]
    fn graph_rem_rel_fail_no_rel() {
        let mut dummy = EntityGraph::new();
        dummy.add_entity("test1", None).unwrap();
        dummy.add_entity("test2", None).unwrap();
        dummy.add_entity("test3", None).unwrap();

        dummy
            .add_relationship(
                "test1",
                RelationshipType::One,
                "test2",
                RelationshipType::One,
            )
            .unwrap();
        assert!(dummy.remove_relationship("test2", "test3").is_err());
        assert_eq!(dummy.get_entity_relationships("test1").unwrap().len(), 1);
        assert_eq!(dummy.get_entity_relationships("test2").unwrap().len(), 1);
    }
}

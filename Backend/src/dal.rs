use redb::{Database, Error, ReadableTable, TableDefinition};

trait dal {
    fn insert(&self);
    fn delete(&self);
    fn update(&self);
    fn search(&self);
    fn get_list(&self);
}

const TABLE: TableDefinition<&str, u64> = TableDefinition::new("my_data");

struct redb_dal {}

// // Convert the data object to a JSON representation
// let json_data = serde_json::to_string(&data_object).unwrap();

// // Store the JSON representation in redb with the 'id' as the key
// redb.set(data_object.id.to_string(), json_data).unwrap();

// // Retrieve the data from redb by its 'id'
// let retrieved_data = redb.get(data_object.id.to_string()).unwrap();
// let parsed_data: DataObject = serde_json::from_str(&retrieved_data).unwrap();

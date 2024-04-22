package dal

import (
	"log"

	"github.com/dgraph-io/badger"
)

func Search(key string) string {
	db, db_err := badger.Open(badger.DefaultOptions("../db_files"))
	if db_err != nil {
		log.Fatal(db_err)
	}
	defer db.Close()

	var valCopy []byte
	search_err := db.View(func(txn *badger.Txn) error {
		item, err := txn.Get([]byte(key))

		if err != nil {
			return err
		}

		err = item.Value(func(val []byte) error {
			valCopy = append([]byte{}, val...)
			return nil
		})

		if err != nil {
			return err
		}

		return nil
	})

	if search_err != nil {
		switch search_err.Error() {
		case "ErrKeyNotFound":
			return ""
		default:
			log.Fatal(search_err)
		}
	}

	return string(valCopy)
}

func Add(key string, value string) error {
	db, err := badger.Open(badger.DefaultOptions("../db_files"))
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	err = db.Update(func(txn *badger.Txn) error {
		err := txn.Set([]byte(key), []byte(value))
		return err
	})

	if err != nil {
		return err
	}

	return nil
}

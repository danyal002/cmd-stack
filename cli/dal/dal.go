package dal

import (
	"log"

	"github.com/dgraph-io/badger"
)

const DATABASE_DIRECTORY = "../db_files"

func Search(key string) string {
	db, db_err := badger.Open(badger.DefaultOptions(DATABASE_DIRECTORY))
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
	db, err := badger.Open(badger.DefaultOptions(DATABASE_DIRECTORY))
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

func GenerateId() uint64 {
	db, db_err := badger.Open(badger.DefaultOptions(DATABASE_DIRECTORY))
	if db_err != nil {
		log.Fatal(db_err)
	}

	defer db.Close()

	seq, seq_err := db.GetSequence([]byte("cmd-id"), 100)
	if seq_err != nil {
		log.Fatal(seq_err)
	}

	defer seq.Release()

	result, res_err := seq.Next()
	if res_err != nil {
		log.Fatal(res_err)
	}

	return result
}

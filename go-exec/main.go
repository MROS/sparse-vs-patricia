package main

import "go-exec/trie"
import "github.com/ethereum/go-ethereum/crypto"
import aergodb "github.com/aergoio/aergo-lib/db"

func main() {
	var db = aergodb.NewDB(aergodb.BadgerImpl, "db-dir/aergoDB")
	trie.NewSMT(make([]byte, 1), crypto.Keccak256, db)
}

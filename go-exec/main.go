package main

import (
	"bufio"
	"encoding/hex"
	"fmt"
	"go-exec/trie"
	"io/ioutil"
	"os"
	"strings"
	"time"

	aergodb "github.com/aergoio/aergo-lib/db"
	"github.com/ethereum/go-ethereum/crypto"
)

type instruction struct {
	name string
	argv [][]byte
}

func readProgram(file *os.File) []instruction {
	var program []instruction
	bufFile := bufio.NewReader(file)
	for {
		line, _, err := bufFile.ReadLine()
		if err != nil {
			break
		}
		units := strings.Split(string(line), " ")
		var argv [][]byte
		switch units[0] {
		case "get":
			key, _ := hex.DecodeString(units[1])
			argv = append(argv, key)
			ins := instruction{
				name: "get",
				argv: argv,
			}
			program = append(program, ins)
		case "insert":
			key, _ := hex.DecodeString(units[1])
			value, _ := hex.DecodeString(units[2])
			argv = append(argv, key, value)
			ins := instruction{
				name: "insert",
				argv: argv,
			}
			program = append(program, ins)
		case "root":
			ins := instruction{
				name: "root",
				argv: argv,
			}
			program = append(program, ins)
		case "flush":
			ins := instruction{
				name: "flush",
				argv: argv,
			}
			program = append(program, ins)
		default:
			fmt.Printf("不知名的指令： %v\n", units[0])
		}
	}
	return program
}

func execute(program []instruction, trie *trie.SMT) {
	start := time.Now()
	var keys [][]byte
	var values [][]byte
	for _, ins := range program {
		switch ins.name {
		case "get":
			// key := hex.EncodeToString(ins.argv[0])
			_, _ = trie.Get(ins.argv[0])
			// value := hex.EncodeToString(result)
			// fmt.Printf("get %v 得到 %v\n", key, value)
		case "insert":
			// key := hex.EncodeToString(ins.argv[0])
			// value := hex.EncodeToString(ins.argv[1])
			// fmt.Printf("insert %v %v\n", key, value)
			// trie.Update([][]byte{ins.argv[0]}, [][]byte{ins.argv[1]})
			keys = append(keys, ins.argv[0])
			values = append(values, ins.argv[1])
		case "root":
			trie.AtomicUpdate(keys, values)
			keys = [][]byte{}
			values = [][]byte{}
			root := hex.EncodeToString(trie.Root)
			fmt.Printf("root: %v\n", root)
		case "flush":
		}
	}
	duration := time.Since(start)
	fmt.Printf("%v 秒\n", duration.Seconds())
}

func main() {
	fileInfos, _ := ioutil.ReadDir("../test_data")
	for _, fileInfo := range fileInfos {
		file, _ := os.Open("../test_data/" + fileInfo.Name())
		program := readProgram(file)
		var db = aergodb.NewDB(aergodb.BadgerImpl, "db-dir/aergoDB")
		smt := trie.NewSMT(nil, crypto.Keccak256, db)
		execute(program, smt)
		db.Close()
	}
}

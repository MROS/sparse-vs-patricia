interface Insert {
	kind: 'insert',
	key: String,
	value: String
}

interface Get {
	kind: 'get',
	key: String
}

interface Commit {
	kind: 'commit',
}

type Instruction = Insert | Get | Commit;

function genTestData(): Instruction[] {
	return [];
}

genTestData();
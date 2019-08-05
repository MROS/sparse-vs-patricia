const KEY_LENGTH: number = 64;

interface Insert {
	kind: 'insert',
	key: String,
	value: String
}

function new_insert(key: String, value: String): Insert {
	return { kind: 'insert', key, value };
}

interface Get {
	kind: 'get',
	key: String
}

function new_get(key: String): Get {
	return { kind: 'get', key };
}

interface Commit {
	kind: 'commit',
}

type Instruction = Insert | Get | Commit;

function randomHex(): string {
	const r_0_1 = Math.random();
	const hex = Math.floor(r_0_1 * 16);
	return hex.toString(16);
}

function randomInsert(): string {
	let ret = '';
	for (let i = 0; i < KEY_LENGTH; i++) {
		ret += randomHex();
	}
	return ret;
}

function genTestData(): Instruction[] {
	console.log(randomInsert());
	return [];
}

genTestData();
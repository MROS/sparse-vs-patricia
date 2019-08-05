const KEY_LENGTH: number = 64;

enum Kind {
	INSERT,
	GET,
	COMMIT
}

interface Instruction {
	kind: Kind
	toString: () => string
}

class Insert {
	kind: Kind;
	key: string;
	value: string;
	constructor(key: string, value: string) {
		this.kind = Kind.INSERT;
		this.key = key;
		this.value = value;
	}
	toString(): string {
		return `insert ${this.key} ${this.value}`;
	}
}

class Get {
	kind: Kind;
	key: string;
	constructor(key: string) {
		this.kind = Kind.GET;
		this.key = key;
	}
	toString(): string {
		return `get ${this.key}`;
	}
}

const commit = {
	kind: Kind.COMMIT,
	toString: () => {
		return 'commit';
	}
};

function randomHex(): string {
	const r_0_1 = Math.random();
	const hex = Math.floor(r_0_1 * 16);
	return hex.toString(16);
}

function randomKey(): string {
	let ret = '';
	for (let i = 0; i < KEY_LENGTH; i++) {
		ret += randomHex();
	}
	return ret;
}

function randomGet(key_pool: string[]): Get {
	const r_0_1 = Math.random();
	const index = Math.floor(r_0_1 * key_pool.length);
	return new Get(key_pool[index]);
}

function randomInsert(): Insert {
	const key = randomKey();
	const value = randomKey();
	return new Insert(key, value);
}

function genTestData(n: number): Instruction[] {
	const data: Instruction[] = [];
	let key_pool: string[] = [];
	// 第一個指令一定是 insert ，樹裡沒有資料無法 get
	const first_insert = randomInsert();
	data.push(first_insert);
	key_pool.push(first_insert.key);

	for (let i = 1; i < n; i++) {
		if (Math.random() < 0.5) {
			data.push(randomGet(key_pool));
		} else {
			const insert = randomInsert();
			data.push(insert);
			key_pool.push(insert.key);
		}
	}
	data.push(commit);
	return data;
}

function main(): void {
	const data = genTestData(10);
	for (let instruction of data) {
		console.log(instruction.toString());
	}
}

main();
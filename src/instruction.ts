const KEY_LENGTH: number = 64;

enum Kind {
	INSERT,
	GET,
	ROOT,
	FLUSH
}

export interface Instruction {
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

const root = {
	kind: Kind.ROOT,
	toString: () => {
		return 'root';
	}
};

const flush = {
	kind: Kind.FLUSH,
	toString: () => {
		return 'flush';
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

export {
	KEY_LENGTH,
	Kind,
	Insert,
	Get,
	root,
	flush,
	randomHex,
	randomKey,
	randomGet,
	randomInsert
};
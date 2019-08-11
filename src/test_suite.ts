import {
	Instruction,
	randomGet,
	randomInsert,
	root,
	flush
} from './instruction';

function randomInsertGet(n: number): [Instruction[], string[]] {
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
	return [data, key_pool];
}

function simple(): Instruction[] {
	const data = randomInsertGet(5)[0];
	data.push(root);
	return data;
}

function flushThenRestore(n: number): Instruction[] {
	const [data, key_pool] = randomInsertGet(n);
	data.push(root);
	data.push(flush);
	for (let i = 1; i < n; i++) {
		if (Math.random() < 0.5) {
			data.push(randomGet(key_pool));
		} else {
			const insert = randomInsert();
			data.push(insert);
			key_pool.push(insert.key);
		}
	}
	data.push(root);
	return data;
}

const test_suite: [Instruction[], string][] = [
	[simple(), 'simple'],
	[flushThenRestore(10000), 'flushThenRestore-10000']
];

export {
	test_suite
};
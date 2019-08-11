import {
	Instruction,
	randomGet,
	randomInsert,
	root,
	flush
} from './instruction';

function randomInsertGet(n: number, key_pool: string[]): [Instruction[], string[]] {
	const data: Instruction[] = [];
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
	const data = randomInsertGet(5, [])[0];
	data.push(root);
	return data;
}

function regularRoot(blockSize: number, rootTimes: number): Instruction[] {
	let [ret_data, ret_key_pool] = randomInsertGet(blockSize, []);
	ret_data.push(root);
	for (let i = 1; i < rootTimes; i++) {
		let [data, key_pool] = randomInsertGet(blockSize, ret_key_pool);
		ret_key_pool = key_pool;
		ret_data = ret_data.concat(data);
		ret_data.push(root);
	}

	return ret_data;
}

function flushThenRestore(n: number): Instruction[] {
	const [data, key_pool] = randomInsertGet(n, []);
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
	[regularRoot(10, 10), 'regularRoot'],
	[flushThenRestore(2), 'flushThenRestore-10000']
];

export {
	test_suite
};
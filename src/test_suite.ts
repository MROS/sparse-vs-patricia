import {
	Instruction,
	randomGet,
	randomInsert,
	root,
	flush
} from './instruction';

function simple(n: number): Instruction[] {
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
	data.push(root);
	return data;
}

const test_suite = [
	() => simple(1000)
];

export {
	test_suite
};
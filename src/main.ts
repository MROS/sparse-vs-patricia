import { test_suite } from './test_suite';
import * as fs from 'fs';

function main(): void {
	for (let test_generator of test_suite) {
		const [program, name] = test_generator;
		console.log(`寫入 test_data/${name}`);
		let fd = fs.openSync(`test_data/${name}`, 'w');
		for (let instruction of program) {
			fs.appendFileSync(fd, instruction.toString() + '\n', 'utf8');
		}
		fs.closeSync(fd);
	}
}

main();
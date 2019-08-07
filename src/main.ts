import { test_suite } from './test_suite';

function main(): void {
	for (let test_generator of test_suite) {
		const program = test_generator();
		for (let instruction of program) {
			console.log(instruction.toString());
		}
	}
}

main();
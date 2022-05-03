import Main from './svelte/Main.svelte';

let containers = ["these", "are", "a", "few", "containers"]
const main = new Main({
	target: document.body,
	props: {
		containers: containers
	}
})

export default main
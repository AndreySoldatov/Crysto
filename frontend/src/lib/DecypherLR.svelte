<script lang="ts">
	import { onMount } from 'svelte';

	function randomString(length = 16) {
		const chars =
			'abcdefghijklmnopqrstuvwxyz' +
			'ABCDEFGHIJKLMNOPQRSTUVWXYZ' +
			'0123456789' +
			'!@#$%^&*()_+-=[]{}|;:,.<>/?';

		const array = new Uint32Array(length);
		crypto.getRandomValues(array);

		return Array.from(array, (x) => chars[x % chars.length]).join('');
	}

	export let classes = '';
	export let value: string = 'Hello World';
	export let duration = 50;

	var currentValue = value;
	var position = value.length;

	let onEnter = () => {
		if (position < value.length) return;

		position = 0;
	};

	onMount(() => {
		const animation = setInterval(() => {
			if (position < value.length) {
				position += 1;
				currentValue = value.slice(0, position) + randomString(value.length - position);
			}
		}, duration);

		return () => clearInterval(animation);
	});
</script>

<p on:mouseenter={onEnter} class="{classes} select-none">{currentValue}</p>
